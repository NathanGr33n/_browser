use wgpu::{Device, Queue, RenderPass, RenderPipeline, Buffer, Texture, Sampler, BindGroup};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use url::Url;
use crate::layout::Rect;
use super::image_cache::{ImageCache, DecodedImage};

/// Vertex data for image rendering (position + tex coords)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ImageVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl ImageVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x2,  // tex_coords
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ImageVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Image drawing command
pub struct ImageCommand {
    pub url: Url,
    pub rect: Rect,
}

/// GPU texture for an image
struct GpuImage {
    texture: Texture,
    bind_group: BindGroup,
}

/// Painter for rendering images with GPU acceleration
pub struct ImagePainter {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: Sampler,
    gpu_images: HashMap<Url, GpuImage>,
    max_images: usize,
    current_commands: Vec<(Url, usize)>, // (URL, vertex_offset)
}

impl ImagePainter {
    /// Create a new image painter
    pub fn new(device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        // Load shader
        let shader_source = include_str!("shaders/image.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Image Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create bind group layout for texture
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Image Bind Group Layout"),
            entries: &[
                // Image texture
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
            label: Some("Image Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Image Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ImageVertex::desc()],
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
                cull_mode: None,
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

        // Create sampler for textures
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Image Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create buffers (preallocate for 1000 images)
        let max_images = 1000;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Image Vertex Buffer"),
            size: (max_images * 4 * std::mem::size_of::<ImageVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Image Index Buffer"),
            size: (max_images * 6 * std::mem::size_of::<u16>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            bind_group_layout,
            sampler,
            gpu_images: HashMap::new(),
            max_images,
            current_commands: Vec::new(),
        }
    }

    /// Upload an image to GPU
    fn upload_image(&mut self, device: &Device, queue: &Queue, decoded: &DecodedImage) {
        // Create texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width: decoded.width,
                height: decoded.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Upload texture data
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &decoded.data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * decoded.width),
                rows_per_image: Some(decoded.height),
            },
            wgpu::Extent3d {
                width: decoded.width,
                height: decoded.height,
                depth_or_array_layers: 1,
            },
        );

        // Create bind group
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Image Bind Group"),
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

        self.gpu_images.insert(
            decoded.url.clone(),
            GpuImage {
                texture,
                bind_group,
            },
        );
    }

    /// Prepare images for rendering
    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        image_cache: &ImageCache,
        commands: &[ImageCommand],
        viewport_size: (u32, u32),
    ) -> usize {
        if commands.is_empty() {
            self.current_commands.clear();
            return 0;
        }

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        self.current_commands.clear();

        for cmd in commands.iter().take(self.max_images) {
            // Get decoded image from cache
            let decoded = match image_cache.get(&cmd.url) {
                Some(img) => img,
                None => continue, // Skip if not in cache
            };

            // Upload to GPU if not already uploaded
            if !self.gpu_images.contains_key(&cmd.url) {
                self.upload_image(device, queue, decoded);
            }

            // Convert screen coordinates to NDC
            let x1 = (cmd.rect.x / viewport_size.0 as f32) * 2.0 - 1.0;
            let y1 = 1.0 - (cmd.rect.y / viewport_size.1 as f32) * 2.0;
            let x2 = ((cmd.rect.x + cmd.rect.width) / viewport_size.0 as f32) * 2.0 - 1.0;
            let y2 = 1.0 - ((cmd.rect.y + cmd.rect.height) / viewport_size.1 as f32) * 2.0;

            // Texture coordinates (0,0) to (1,1)
            let base_index = vertices.len() as u16;
            vertices.extend_from_slice(&[
                ImageVertex { position: [x1, y1], tex_coords: [0.0, 0.0] }, // Top-left
                ImageVertex { position: [x2, y1], tex_coords: [1.0, 0.0] }, // Top-right
                ImageVertex { position: [x2, y2], tex_coords: [1.0, 1.0] }, // Bottom-right
                ImageVertex { position: [x1, y2], tex_coords: [0.0, 1.0] }, // Bottom-left
            ]);

            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);

            self.current_commands.push((cmd.url.clone(), (vertices.len() - 4) / 4));
        }

        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
        }

        self.current_commands.len()
    }

    /// Render the prepared images
    pub fn render<'rpass>(&'rpass self, render_pass: &mut RenderPass<'rpass>) {
        if self.current_commands.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Render each image with its own bind group
        for (i, (url, _vertex_offset)) in self.current_commands.iter().enumerate() {
            if let Some(gpu_image) = self.gpu_images.get(url) {
                render_pass.set_bind_group(0, &gpu_image.bind_group, &[]);
                let index_start = (i * 6) as u32;
                let index_end = index_start + 6;
                render_pass.draw_indexed(index_start..index_end, 0, 0..1);
            }
        }
    }

    /// Clear GPU image cache (useful for memory management)
    pub fn clear_gpu_cache(&mut self) {
        self.gpu_images.clear();
    }
}
