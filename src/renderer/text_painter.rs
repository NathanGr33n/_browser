use wgpu::{Device, Queue, RenderPass, RenderPipeline, Buffer, Texture, Sampler, BindGroup};
use bytemuck::{Pod, Zeroable};
use crate::css::Color;
use crate::layout::Rect;
use super::text_renderer::TextRenderer;
use super::glyph_cache::GlyphKey;

/// Vertex data for text rendering (position + tex coords + color)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct TextVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4],
}

impl TextVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x2,  // tex_coords
        2 => Float32x4,  // color
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Text drawing command
pub struct TextCommand {
    pub text: String,
    pub rect: Rect,
    pub color: Color,
    pub font_family: String,
    pub font_size: f32,
}

/// Painter for rendering text with GPU acceleration
pub struct TextPainter {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    bind_group: Option<BindGroup>,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: Sampler,
    max_glyphs: usize,
    glyph_count: usize,
}

impl TextPainter {
    /// Create a new text painter
    pub fn new(device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        // Load shader
        let shader_source = include_str!("shaders/text.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create bind group layout for texture
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Text Bind Group Layout"),
            entries: &[
                // Glyph atlas texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TextVertex::desc()],
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
                cull_mode: None, // Don't cull for text
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

        // Create sampler for texture
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Glyph Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create buffers (preallocate for 10000 glyphs)
        let max_glyphs = 10000;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Vertex Buffer"),
            size: (max_glyphs * 4 * std::mem::size_of::<TextVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Index Buffer"),
            size: (max_glyphs * 6 * std::mem::size_of::<u16>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            bind_group: None,
            bind_group_layout,
            sampler,
            max_glyphs,
            glyph_count: 0,
        }
    }

    /// Update the atlas texture binding
    pub fn update_atlas(&mut self, device: &Device, atlas_texture: &Texture) {
        let texture_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        self.bind_group = Some(bind_group);
    }

    /// Prepare text for rendering
    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        text_renderer: &mut TextRenderer,
        commands: &[TextCommand],
        viewport_size: (u32, u32),
    ) -> usize {
        if commands.is_empty() {
            self.glyph_count = 0;
            return 0;
        }

        // Upload atlas if needed
        text_renderer.upload_atlas(device, queue);

        // Update bind group if we have an atlas texture
        if let Some(atlas_texture) = text_renderer.atlas_texture() {
            if self.bind_group.is_none() {
                self.update_atlas(device, atlas_texture);
            }
        }

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut total_glyphs = 0;

        let atlas_dims = text_renderer.glyph_cache().atlas_dimensions();

        for cmd in commands {
            if total_glyphs >= self.max_glyphs {
                break;
            }

            // Get font
            let font = text_renderer.font_manager_mut().get_font(&cmd.font_family);
            let font_id = text_renderer.glyph_cache_mut().register_font(font);

            let mut x_offset = cmd.rect.x;
            let y_offset = cmd.rect.y;

            // Convert color
            let color_f = [
                cmd.color.r as f32 / 255.0,
                cmd.color.g as f32 / 255.0,
                cmd.color.b as f32 / 255.0,
                cmd.color.a as f32 / 255.0,
            ];

            // Render each character
            for ch in cmd.text.chars() {
                let key = GlyphKey {
                    character: ch,
                    size: cmd.font_size as u32,
                    font_id,
                };

                if let Some(glyph) = text_renderer.glyph_cache_mut().get_or_rasterize(key) {
                    // Skip empty glyphs (spaces)
                    if glyph.width > 0 && glyph.height > 0 {
                        // Screen coordinates
                        let x1 = x_offset + glyph.bearing_x;
                        let y1 = y_offset + glyph.bearing_y;
                        let x2 = x1 + glyph.width as f32;
                        let y2 = y1 + glyph.height as f32;

                        // Convert to NDC
                        let ndc_x1 = (x1 / viewport_size.0 as f32) * 2.0 - 1.0;
                        let ndc_y1 = 1.0 - (y1 / viewport_size.1 as f32) * 2.0;
                        let ndc_x2 = (x2 / viewport_size.0 as f32) * 2.0 - 1.0;
                        let ndc_y2 = 1.0 - (y2 / viewport_size.1 as f32) * 2.0;

                        // Texture coordinates
                        let u1 = glyph.atlas_x as f32 / atlas_dims.0 as f32;
                        let v1 = glyph.atlas_y as f32 / atlas_dims.1 as f32;
                        let u2 = (glyph.atlas_x + glyph.width) as f32 / atlas_dims.0 as f32;
                        let v2 = (glyph.atlas_y + glyph.height) as f32 / atlas_dims.1 as f32;

                        // Create vertices
                        let base_index = vertices.len() as u16;
                        vertices.extend_from_slice(&[
                            TextVertex { position: [ndc_x1, ndc_y1], tex_coords: [u1, v1], color: color_f },
                            TextVertex { position: [ndc_x2, ndc_y1], tex_coords: [u2, v1], color: color_f },
                            TextVertex { position: [ndc_x2, ndc_y2], tex_coords: [u2, v2], color: color_f },
                            TextVertex { position: [ndc_x1, ndc_y2], tex_coords: [u1, v2], color: color_f },
                        ]);

                        // Create indices
                        indices.extend_from_slice(&[
                            base_index,
                            base_index + 1,
                            base_index + 2,
                            base_index,
                            base_index + 2,
                            base_index + 3,
                        ]);

                        total_glyphs += 1;
                    }

                    x_offset += glyph.advance;
                }
            }
        }

        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
        }

        self.glyph_count = total_glyphs;
        total_glyphs
    }

    /// Render the prepared text
    pub fn render<'rpass>(&'rpass self, render_pass: &mut RenderPass<'rpass>) {
        if self.glyph_count == 0 || self.bind_group.is_none() {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(self.glyph_count * 6) as u32, 0, 0..1);
    }
}
