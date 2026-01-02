mod painter;
mod border_painter;
mod text_painter;
pub mod font_manager;
pub mod glyph_cache;
pub mod text_renderer;
pub mod image_cache;

use wgpu::{
    Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, TextureFormat,
    PresentMode, CompositeAlphaMode,
};
use winit::window::Window;
use std::sync::Arc;

pub use painter::RectPainter;
pub use border_painter::BorderPainter;
pub use text_painter::TextPainter;
use crate::css::Color;
use crate::layout::Rect;

/// GPU-accelerated renderer using wgpu
pub struct Renderer<'window> {
    surface: Surface<'window>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: (u32, u32),
    rect_painter: RectPainter,
    border_painter: BorderPainter,
}

impl<'window> Renderer<'window> {
    /// Initialize the renderer for the given window
    /// 
    /// This sets up the GPU surface, adapter, and device
    pub async fn new(window: &'window Arc<Window>) -> Result<Self, RendererError> {
        let size = window.inner_size();
        
        // Create wgpu instance with default backends
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface for the window
        // SAFETY: The window must live as long as the surface
        let surface = instance
            .create_surface(window)
            .map_err(|e| RendererError::Initialization(format!("Failed to create surface: {}", e)))?;

        // Request an adapter (represents a physical GPU)
        let adapter = Self::request_adapter(&instance, &surface).await?;

        // Request a device and queue (logical GPU interface)
        let (device, queue) = Self::request_device(&adapter).await?;

        // Configure the surface for rendering
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: PresentMode::Fifo, // VSync
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Create painters
        let rect_painter = RectPainter::new(&device, surface_format);
        let border_painter = BorderPainter::new(&device, surface_format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (size.width, size.height),
            rect_painter,
            border_painter,
        })
    }

    /// Request a GPU adapter with fallback options
    async fn request_adapter(
        instance: &Instance,
        surface: &Surface<'window>,
    ) -> Result<Adapter, RendererError> {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RendererError::Initialization(
                "Failed to find suitable GPU adapter".to_string(),
            ))
    }

    /// Request a logical device from the adapter
    async fn request_device(adapter: &Adapter) -> Result<(Device, Queue), RendererError> {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Browser Engine Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| {
                RendererError::Initialization(format!("Failed to create device: {}", e))
            })
    }

    /// Resize the render surface
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Get the current surface size
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Get the surface format
    pub fn format(&self) -> TextureFormat {
        self.config.format
    }

    /// Get a reference to the device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get a reference to the queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Begin a new frame
    /// 
    /// Returns the current surface texture to render to
    pub fn begin_frame(&self) -> Result<wgpu::SurfaceTexture, RendererError> {
        self.surface
            .get_current_texture()
            .map_err(|e| RendererError::Frame(format!("Failed to acquire surface texture: {}", e)))
    }

    /// Render a frame with the provided callback
    /// 
    /// The callback receives a command encoder to record rendering commands
    pub fn render<F>(&self, mut render_fn: F) -> Result<(), RendererError>
    where
        F: FnMut(&Device, &Queue, &wgpu::TextureView, &mut wgpu::CommandEncoder),
    {
        // Get the current frame's texture
        let frame = self.begin_frame()?;
        
        // Create a view of the texture for rendering
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder to record GPU commands
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Execute the render function
        render_fn(&self.device, &self.queue, &view, &mut encoder);

        // Submit commands to the GPU
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Present the frame to the screen
        frame.present();

        Ok(())
    }

    /// Clear the screen with the given color
    pub fn clear(&self, color: [f64; 4]) -> Result<(), RendererError> {
        self.render(|_device, _queue, view, encoder| {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color[0],
                            g: color[1],
                            b: color[2],
                            a: color[3],
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        })
    }

    /// Render rectangles from a display list
    pub fn render_rects(&mut self, rects: &[(Rect, Color)]) -> Result<(), RendererError> {
        // Prepare rectangle data
        self.rect_painter.prepare(&self.device, &self.queue, rects, self.size);

        // Render
        self.render(|_device, _queue, view, encoder| {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Rectangle Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.rect_painter.render(&mut render_pass);
        })
    }

    /// Render rectangles and borders together
    pub fn render_rects_and_borders(
        &mut self,
        rects: &[(Rect, Color)],
        borders: &[(Rect, Color, (f32, f32, f32, f32))],
    ) -> Result<(), RendererError> {
        // Prepare data
        self.rect_painter.prepare(&self.device, &self.queue, rects, self.size);
        self.border_painter.prepare(&self.device, &self.queue, borders, self.size);

        // Render both in same pass
        self.render(|_device, _queue, view, encoder| {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Combined Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Render backgrounds first, then borders on top
            self.rect_painter.render(&mut render_pass);
            self.border_painter.render(&mut render_pass);
        })
    }
}

/// Renderer errors
#[derive(Debug)]
pub enum RendererError {
    /// Error during renderer initialization
    Initialization(String),
    /// Error during frame rendering
    Frame(String),
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::Initialization(msg) => write!(f, "Renderer initialization error: {}", msg),
            RendererError::Frame(msg) => write!(f, "Frame rendering error: {}", msg),
        }
    }
}

impl std::error::Error for RendererError {}
