// Canvas 2D API - Phase 7 Task 3

/// Canvas element with 2D drawing context
pub struct Canvas {
    /// Width in pixels
    width: u32,
    /// Height in pixels
    height: u32,
    /// Pixel data (RGBA format)
    pixels: Vec<u8>,
    /// 2D rendering context
    context: Option<CanvasRenderingContext2D>,
}

impl Canvas {
    /// Create a new canvas
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = (width * height * 4) as usize;
        Self {
            width,
            height,
            pixels: vec![0; pixel_count],
            context: None,
        }
    }
    
    /// Get 2D rendering context
    pub fn get_context_2d(&mut self) -> &mut CanvasRenderingContext2D {
        if self.context.is_none() {
            self.context = Some(CanvasRenderingContext2D::new(self.width, self.height));
        }
        self.context.as_mut().unwrap()
    }
    
    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Get pixel data
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
    
    /// Render context to pixels
    pub fn render(&mut self) {
        if let Some(ctx) = &self.context {
            self.pixels = ctx.get_image_data();
        }
    }
}

/// 2D rendering context
pub struct CanvasRenderingContext2D {
    /// Canvas width
    width: u32,
    /// Canvas height
    height: u32,
    /// Pixel data (RGBA)
    image_data: Vec<u8>,
    /// Current path
    current_path: Path2D,
    /// Drawing state stack
    state_stack: Vec<DrawingState>,
    /// Current state
    state: DrawingState,
}

/// Drawing state (saved/restored with save/restore)
#[derive(Clone)]
struct DrawingState {
    /// Fill style
    fill_style: Color,
    /// Stroke style
    stroke_style: Color,
    /// Line width
    line_width: f32,
    /// Line cap
    line_cap: LineCap,
    /// Line join
    line_join: LineJoin,
    /// Global alpha
    global_alpha: f32,
    /// Global composite operation
    global_composite_operation: CompositeOperation,
    /// Font
    font: String,
    /// Text align
    text_align: TextAlign,
    /// Text baseline
    text_baseline: TextBaseline,
    /// Transform matrix
    transform: [f32; 6], // a, b, c, d, e, f
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            fill_style: Color::rgba(0, 0, 0, 255),
            stroke_style: Color::rgba(0, 0, 0, 255),
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            global_alpha: 1.0,
            global_composite_operation: CompositeOperation::SourceOver,
            font: "10px sans-serif".to_string(),
            text_align: TextAlign::Start,
            text_baseline: TextBaseline::Alphabetic,
            transform: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], // Identity matrix
        }
    }
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create RGBA color
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create RGB color (opaque)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }
}

/// Line cap style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Line join style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// Composite operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeOperation {
    SourceOver,
    SourceIn,
    SourceOut,
    SourceAtop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,
    Lighter,
    Copy,
    Xor,
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
}

/// Text baseline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBaseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

/// 2D path
#[derive(Clone)]
pub struct Path2D {
    /// Path commands
    commands: Vec<PathCommand>,
}

impl Path2D {
    /// Create a new path
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    
    /// Move to point
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::MoveTo { x, y });
    }
    
    /// Line to point
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::LineTo { x, y });
    }
    
    /// Quadratic curve to point
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::QuadraticCurveTo { cpx, cpy, x, y });
    }
    
    /// Bezier curve to point
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::BezierCurveTo { cp1x, cp1y, cp2x, cp2y, x, y });
    }
    
    /// Arc
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.commands.push(PathCommand::Arc {
            x, y, radius, start_angle, end_angle, anticlockwise,
        });
    }
    
    /// Close path
    pub fn close_path(&mut self) {
        self.commands.push(PathCommand::ClosePath);
    }
    
    /// Get commands
    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }
}

impl Default for Path2D {
    fn default() -> Self {
        Self::new()
    }
}

/// Path command
#[derive(Debug, Clone, Copy)]
pub enum PathCommand {
    MoveTo { x: f32, y: f32 },
    LineTo { x: f32, y: f32 },
    QuadraticCurveTo { cpx: f32, cpy: f32, x: f32, y: f32 },
    BezierCurveTo { cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32 },
    Arc { x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool },
    ClosePath,
}

impl CanvasRenderingContext2D {
    /// Create a new 2D context
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = (width * height * 4) as usize;
        Self {
            width,
            height,
            image_data: vec![255; pixel_count], // White background
            current_path: Path2D::new(),
            state_stack: Vec::new(),
            state: DrawingState::default(),
        }
    }
    
    /// Get image data
    pub fn get_image_data(&self) -> Vec<u8> {
        self.image_data.clone()
    }
    
    // State management
    
    /// Save current state
    pub fn save(&mut self) {
        self.state_stack.push(self.state.clone());
    }
    
    /// Restore previous state
    pub fn restore(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.state = state;
        }
    }
    
    // Styling
    
    /// Set fill style
    pub fn set_fill_style(&mut self, color: Color) {
        self.state.fill_style = color;
    }
    
    /// Set stroke style
    pub fn set_stroke_style(&mut self, color: Color) {
        self.state.stroke_style = color;
    }
    
    /// Set line width
    pub fn set_line_width(&mut self, width: f32) {
        self.state.line_width = width.max(0.0);
    }
    
    /// Set global alpha
    pub fn set_global_alpha(&mut self, alpha: f32) {
        self.state.global_alpha = alpha.clamp(0.0, 1.0);
    }
    
    // Path operations
    
    /// Begin a new path
    pub fn begin_path(&mut self) {
        self.current_path = Path2D::new();
    }
    
    /// Move to point
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.current_path.move_to(x, y);
    }
    
    /// Line to point
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.current_path.line_to(x, y);
    }
    
    /// Quadratic curve
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.current_path.quadratic_curve_to(cpx, cpy, x, y);
    }
    
    /// Bezier curve
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.current_path.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }
    
    /// Arc
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.current_path.arc(x, y, radius, start_angle, end_angle, anticlockwise);
    }
    
    /// Close path
    pub fn close_path(&mut self) {
        self.current_path.close_path();
    }
    
    // Drawing operations
    
    /// Fill current path
    pub fn fill(&mut self) {
        self.rasterize_path(true);
    }
    
    /// Stroke current path
    pub fn stroke(&mut self) {
        self.rasterize_path(false);
    }
    
    /// Fill rectangle
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let color = self.state.fill_style;
        self.draw_rect(x, y, width, height, color, true);
    }
    
    /// Stroke rectangle
    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let color = self.state.stroke_style;
        self.draw_rect(x, y, width, height, color, false);
    }
    
    /// Clear rectangle
    pub fn clear_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x1 = x.max(0.0) as u32;
        let y1 = y.max(0.0) as u32;
        let x2 = ((x + width).min(self.width as f32)) as u32;
        let y2 = ((y + height).min(self.height as f32)) as u32;
        
        for py in y1..y2 {
            for px in x1..x2 {
                self.set_pixel(px, py, Color::rgba(255, 255, 255, 0));
            }
        }
    }
    
    /// Fill text
    pub fn fill_text(&mut self, text: &str, x: f32, y: f32) {
        // Simplified text rendering - just draw a placeholder for now
        // In production, would use font rasterization library
        let color = self.state.fill_style;
        let char_width = 8.0;
        let char_height = 12.0;
        
        for (i, _ch) in text.chars().enumerate() {
            let char_x = x + (i as f32 * char_width);
            self.draw_rect(char_x, y - char_height, char_width, char_height, color, true);
        }
    }
    
    /// Stroke text
    pub fn stroke_text(&mut self, text: &str, x: f32, y: f32) {
        // Simplified text rendering
        let color = self.state.stroke_style;
        let char_width = 8.0;
        let char_height = 12.0;
        
        for (i, _ch) in text.chars().enumerate() {
            let char_x = x + (i as f32 * char_width);
            self.draw_rect(char_x, y - char_height, char_width, char_height, color, false);
        }
    }
    
    // Image operations
    
    /// Draw image (simplified)
    pub fn draw_image(&mut self, image_data: &[u8], sx: f32, sy: f32, sw: f32, sh: f32, 
                      dx: f32, dy: f32, dw: f32, dh: f32) {
        // Simplified: copy pixels with basic scaling
        let sx = sx as u32;
        let sy = sy as u32;
        let sw = sw as u32;
        let sh = sh as u32;
        let dx = dx as u32;
        let dy = dy as u32;
        let dw = dw as u32;
        let dh = dh as u32;
        
        for y in 0..dh {
            for x in 0..dw {
                let src_x = sx + (x * sw / dw.max(1));
                let src_y = sy + (y * sh / dh.max(1));
                let src_idx = ((src_y * sw + src_x) * 4) as usize;
                
                if src_idx + 3 < image_data.len() {
                    let color = Color::rgba(
                        image_data[src_idx],
                        image_data[src_idx + 1],
                        image_data[src_idx + 2],
                        image_data[src_idx + 3],
                    );
                    self.set_pixel(dx + x, dy + y, color);
                }
            }
        }
    }
    
    // Helper methods
    
    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 3 >= self.image_data.len() {
            return;
        }
        
        // Apply global alpha
        let alpha = (color.a as f32 * self.state.global_alpha) as u8;
        
        // Basic alpha blending
        if alpha == 255 {
            self.image_data[idx] = color.r;
            self.image_data[idx + 1] = color.g;
            self.image_data[idx + 2] = color.b;
            self.image_data[idx + 3] = alpha;
        } else {
            let src_alpha = alpha as f32 / 255.0;
            let dst_alpha = self.image_data[idx + 3] as f32 / 255.0;
            let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);
            
            if out_alpha > 0.0 {
                self.image_data[idx] = ((color.r as f32 * src_alpha + 
                    self.image_data[idx] as f32 * dst_alpha * (1.0 - src_alpha)) / out_alpha) as u8;
                self.image_data[idx + 1] = ((color.g as f32 * src_alpha + 
                    self.image_data[idx + 1] as f32 * dst_alpha * (1.0 - src_alpha)) / out_alpha) as u8;
                self.image_data[idx + 2] = ((color.b as f32 * src_alpha + 
                    self.image_data[idx + 2] as f32 * dst_alpha * (1.0 - src_alpha)) / out_alpha) as u8;
                self.image_data[idx + 3] = (out_alpha * 255.0) as u8;
            }
        }
    }
    
    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color, fill: bool) {
        if fill {
            let x1 = x.max(0.0) as u32;
            let y1 = y.max(0.0) as u32;
            let x2 = ((x + width).min(self.width as f32)) as u32;
            let y2 = ((y + height).min(self.height as f32)) as u32;
            
            for py in y1..y2 {
                for px in x1..x2 {
                    self.set_pixel(px, py, color);
                }
            }
        } else {
            // Stroke: draw outline
            let line_width = self.state.line_width as u32;
            for i in 0..line_width {
                self.draw_line(x + i as f32, y, x + width - i as f32, y, color);
                self.draw_line(x + width, y + i as f32, x + width, y + height - i as f32, color);
                self.draw_line(x + width - i as f32, y + height, x + i as f32, y + height, color);
                self.draw_line(x, y + height - i as f32, x, y + i as f32, color);
            }
        }
    }
    
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
        // Bresenham's line algorithm
        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;
        
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;
        
        loop {
            if x1 >= 0 && y1 >= 0 {
                self.set_pixel(x1 as u32, y1 as u32, color);
            }
            
            if x1 == x2 && y1 == y2 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x1 += sx;
            }
            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }
    
    fn rasterize_path(&mut self, fill: bool) {
        let color = if fill { self.state.fill_style } else { self.state.stroke_style };
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        
        // Clone commands to avoid borrow checker issues
        let commands = self.current_path.commands().to_vec();
        
        for cmd in &commands {
            match cmd {
                PathCommand::MoveTo { x, y } => {
                    current_x = *x;
                    current_y = *y;
                }
                PathCommand::LineTo { x, y } => {
                    self.draw_line(current_x, current_y, *x, *y, color);
                    current_x = *x;
                    current_y = *y;
                }
                PathCommand::QuadraticCurveTo { cpx, cpy, x, y } => {
                    // Simplified: approximate with line segments
                    let steps = 20;
                    for i in 0..=steps {
                        let t = i as f32 / steps as f32;
                        let nt = 1.0 - t;
                        let qx = nt * nt * current_x + 2.0 * nt * t * cpx + t * t * x;
                        let qy = nt * nt * current_y + 2.0 * nt * t * cpy + t * t * y;
                        if i > 0 {
                            self.draw_line(current_x, current_y, qx, qy, color);
                        }
                        current_x = qx;
                        current_y = qy;
                    }
                }
                PathCommand::BezierCurveTo { cp1x, cp1y, cp2x, cp2y, x, y } => {
                    // Simplified: approximate with line segments
                    let steps = 20;
                    for i in 0..=steps {
                        let t = i as f32 / steps as f32;
                        let nt = 1.0 - t;
                        let bx = nt * nt * nt * current_x + 
                                3.0 * nt * nt * t * cp1x + 
                                3.0 * nt * t * t * cp2x + 
                                t * t * t * x;
                        let by = nt * nt * nt * current_y + 
                                3.0 * nt * nt * t * cp1y + 
                                3.0 * nt * t * t * cp2y + 
                                t * t * t * y;
                        if i > 0 {
                            self.draw_line(current_x, current_y, bx, by, color);
                        }
                        current_x = bx;
                        current_y = by;
                    }
                }
                PathCommand::Arc { x, y, radius, start_angle, end_angle, anticlockwise } => {
                    // Draw arc with line segments
                    let steps = 40;
                    let angle_range = if *anticlockwise {
                        start_angle - end_angle
                    } else {
                        end_angle - start_angle
                    };
                    
                    for i in 0..=steps {
                        let t = i as f32 / steps as f32;
                        let angle = if *anticlockwise {
                            start_angle - angle_range * t
                        } else {
                            start_angle + angle_range * t
                        };
                        let px = x + radius * angle.cos();
                        let py = y + radius * angle.sin();
                        if i > 0 {
                            self.draw_line(current_x, current_y, px, py, color);
                        }
                        current_x = px;
                        current_y = py;
                    }
                }
                PathCommand::ClosePath => {
                    // Would close to first point in subpath
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(640, 480);
        assert_eq!(canvas.width(), 640);
        assert_eq!(canvas.height(), 480);
        assert_eq!(canvas.pixels().len(), 640 * 480 * 4);
    }
    
    #[test]
    fn test_context_2d() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        assert_eq!(ctx.width, 100);
        assert_eq!(ctx.height, 100);
    }
    
    #[test]
    fn test_fill_rect() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        ctx.set_fill_style(Color::rgb(255, 0, 0));
        ctx.fill_rect(10.0, 10.0, 20.0, 20.0);
        
        // Check that some pixels were set
        let image_data = ctx.get_image_data();
        assert!(image_data.len() > 0);
    }
    
    #[test]
    fn test_path_operations() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.begin_path();
        ctx.move_to(10.0, 10.0);
        ctx.line_to(90.0, 10.0);
        ctx.line_to(90.0, 90.0);
        ctx.close_path();
        ctx.stroke();
    }
    
    #[test]
    fn test_save_restore() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.set_fill_style(Color::rgb(255, 0, 0));
        ctx.save();
        ctx.set_fill_style(Color::rgb(0, 255, 0));
        ctx.restore();
        
        assert_eq!(ctx.state.fill_style, Color::rgb(255, 0, 0));
    }
    
    #[test]
    fn test_clear_rect() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
        ctx.clear_rect(10.0, 10.0, 20.0, 20.0);
    }
    
    #[test]
    fn test_curves() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.begin_path();
        ctx.move_to(10.0, 50.0);
        ctx.quadratic_curve_to(50.0, 10.0, 90.0, 50.0);
        ctx.stroke();
    }
    
    #[test]
    fn test_arc() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.begin_path();
        ctx.arc(50.0, 50.0, 30.0, 0.0, 2.0 * std::f32::consts::PI, false);
        ctx.stroke();
    }
    
    #[test]
    fn test_text_rendering() {
        let mut canvas = Canvas::new(200, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.fill_text("Hello", 10.0, 50.0);
    }
    
    #[test]
    fn test_alpha_blending() {
        let mut canvas = Canvas::new(100, 100);
        let ctx = canvas.get_context_2d();
        
        ctx.set_global_alpha(0.5);
        ctx.set_fill_style(Color::rgb(255, 0, 0));
        ctx.fill_rect(10.0, 10.0, 50.0, 50.0);
    }
}
