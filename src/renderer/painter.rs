use wgpu::{Device, Queue, RenderPass, RenderPipeline, Buffer};
use bytemuck::{Pod, Zeroable};
use crate::css::Color;
use crate::layout::Rect;

/// Vertex data for a rectangle
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

/// Painter for rendering rectangles with GPU acceleration
pub struct RectPainter {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    max_rects: usize,
    rect_count: usize,
}

impl RectPainter {
    /// Create a new rectangle painter
    pub fn new(device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        // Shader source code
        let shader_source = include_str!("shaders/rect.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rectangle Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rectangle Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rectangle Pipeline"),
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

        // Create buffers (preallocate for 1000 rectangles)
        let max_rects = 1000;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rectangle Vertex Buffer"),
            size: (max_rects * 4 * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rectangle Index Buffer"),
            size: (max_rects * 6 * std::mem::size_of::<u16>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            max_rects,
            rect_count: 0,
        }
    }

    /// Prepare rectangles for rendering
    /// 
    /// Returns the number of rectangles to render
    pub fn prepare(&mut self, _device: &Device, queue: &Queue, rects: &[(Rect, Color)], viewport_size: (u32, u32)) -> usize {
        if rects.is_empty() {
            self.rect_count = 0;
            return 0;
        }

        let count = rects.len().min(self.max_rects);
        let mut vertices = Vec::with_capacity(count * 4);
        let mut indices = Vec::with_capacity(count * 6);

        for (i, (rect, color)) in rects.iter().take(count).enumerate() {
            // Convert screen coordinates to NDC (Normalized Device Coordinates)
            // NDC ranges from -1.0 to 1.0
            let x1 = (rect.x / viewport_size.0 as f32) * 2.0 - 1.0;
            let y1 = 1.0 - (rect.y / viewport_size.1 as f32) * 2.0;
            let x2 = ((rect.x + rect.width) / viewport_size.0 as f32) * 2.0 - 1.0;
            let y2 = 1.0 - ((rect.y + rect.height) / viewport_size.1 as f32) * 2.0;

            // Convert color to normalized float
            let color_f = [
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            ];

            // Create 4 vertices for the rectangle
            let base_index = (i * 4) as u16;
            vertices.extend_from_slice(&[
                Vertex { position: [x1, y1], color: color_f }, // Top-left
                Vertex { position: [x2, y1], color: color_f }, // Top-right
                Vertex { position: [x2, y2], color: color_f }, // Bottom-right
                Vertex { position: [x1, y2], color: color_f }, // Bottom-left
            ]);

            // Create indices for two triangles
            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);
        }

        // Upload to GPU
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));

        self.rect_count = count;
        count
    }

    /// Render the prepared rectangles
    pub fn render<'rpass>(&'rpass self, render_pass: &mut RenderPass<'rpass>) {
        if self.rect_count == 0 {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(self.rect_count * 6) as u32, 0, 0..1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        // Ensure vertex is properly packed
        assert_eq!(std::mem::size_of::<Vertex>(), 24);
    }
}
