use wgpu::{Device, Queue, RenderPass, RenderPipeline, Buffer};
use bytemuck::{Pod, Zeroable};
use crate::css::Color;
use crate::layout::Rect;

/// Vertex data for a border edge
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x4,  // color
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Painter for rendering borders with GPU acceleration
pub struct BorderPainter {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    max_borders: usize,
    border_count: usize,
}

impl BorderPainter {
    /// Create a new border painter
    pub fn new(device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        // Reuse the same shader as rectangles
        let shader_source = include_str!("shaders/rect.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Border Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Border Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Border Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create buffers (preallocate for 500 borders, 4 edges each = 2000 rectangles)
        let max_borders = 500;
        let max_rects = max_borders * 4; // 4 edges per border
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Border Vertex Buffer"),
            size: (max_rects * 4 * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Border Index Buffer"),
            size: (max_rects * 6 * std::mem::size_of::<u16>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            max_borders,
            border_count: 0,
        }
    }

    /// Prepare borders for rendering
    /// 
    /// Border widths are (left, right, top, bottom)
    pub fn prepare(
        &mut self,
        _device: &Device,
        queue: &Queue,
        borders: &[(Rect, Color, (f32, f32, f32, f32))],
        viewport_size: (u32, u32),
    ) -> usize {
        if borders.is_empty() {
            self.border_count = 0;
            return 0;
        }

        let count = borders.len().min(self.max_borders);
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for (_border_idx, (rect, color, widths)) in borders.iter().take(count).enumerate() {
            let (left_w, right_w, top_w, bottom_w) = widths;

            // Convert color to normalized float
            let color_f = [
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            ];

            // Create rectangles for each border edge
            // Each border generates up to 4 rectangles (one per edge)
            
            // Top border
            if *top_w > 0.0 {
                let top_rect = Rect {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: *top_w,
                };
                add_rect_vertices(&mut vertices, &mut indices, &top_rect, color_f, viewport_size);
            }

            // Bottom border
            if *bottom_w > 0.0 {
                let bottom_rect = Rect {
                    x: rect.x,
                    y: rect.y + rect.height - bottom_w,
                    width: rect.width,
                    height: *bottom_w,
                };
                add_rect_vertices(&mut vertices, &mut indices, &bottom_rect, color_f, viewport_size);
            }

            // Left border (between top and bottom)
            if *left_w > 0.0 {
                let left_rect = Rect {
                    x: rect.x,
                    y: rect.y + top_w,
                    width: *left_w,
                    height: rect.height - top_w - bottom_w,
                };
                add_rect_vertices(&mut vertices, &mut indices, &left_rect, color_f, viewport_size);
            }

            // Right border (between top and bottom)
            if *right_w > 0.0 {
                let right_rect = Rect {
                    x: rect.x + rect.width - right_w,
                    y: rect.y + top_w,
                    width: *right_w,
                    height: rect.height - top_w - bottom_w,
                };
                add_rect_vertices(&mut vertices, &mut indices, &right_rect, color_f, viewport_size);
            }
        }

        // Upload to GPU
        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
        }

        self.border_count = indices.len() / 6; // Number of rectangles
        count
    }

    /// Render the prepared borders
    pub fn render<'rpass>(&'rpass self, render_pass: &mut RenderPass<'rpass>) {
        if self.border_count == 0 {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(self.border_count * 6) as u32, 0, 0..1);
    }
}

/// Helper to add rectangle vertices for a border edge
fn add_rect_vertices(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    rect: &Rect,
    color: [f32; 4],
    viewport_size: (u32, u32),
) {
    // Convert to NDC
    let x1 = (rect.x / viewport_size.0 as f32) * 2.0 - 1.0;
    let y1 = 1.0 - (rect.y / viewport_size.1 as f32) * 2.0;
    let x2 = ((rect.x + rect.width) / viewport_size.0 as f32) * 2.0 - 1.0;
    let y2 = 1.0 - ((rect.y + rect.height) / viewport_size.1 as f32) * 2.0;

    let base_index = vertices.len() as u16;

    // Add 4 vertices
    vertices.extend_from_slice(&[
        Vertex { position: [x1, y1], color },
        Vertex { position: [x2, y1], color },
        Vertex { position: [x2, y2], color },
        Vertex { position: [x1, y2], color },
    ]);

    // Add 6 indices for 2 triangles
    indices.extend_from_slice(&[
        base_index,
        base_index + 1,
        base_index + 2,
        base_index,
        base_index + 2,
        base_index + 3,
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        assert_eq!(std::mem::size_of::<Vertex>(), 24);
    }
}
