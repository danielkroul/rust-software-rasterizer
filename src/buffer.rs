use crate::drawable::Drawable;

/// A simple framebuffer storing pixel data in a contiguous `Vec<u32>`.
/// 
/// Provides basic operations like setting pixels, clearing, resizing, 
/// and accessing the data slice.
pub struct Buffer {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Buffer {
    /// Create a new Buffer with the given width and height, initialized to zero.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    /// Get the buffer's width in pixels.
    pub fn width(&self) -> usize { self.width }

    /// Get the buffer's height in pixels.
    pub fn height(&self) -> usize { self.height }

    /// Resize the framebuffer and update the length of the data.
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        self.data.resize(new_width * new_height, 0);
    }

    /// Set a pixel at (x, y) to a given color.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }

    /// Clear the entire buffer to a given color.
    pub fn clear(&mut self, color: u32) {
        self.data.fill(color);
    }

    /// Get a slice reference to a buffers pixel data.
    pub fn as_slice(&self) -> &[u32] {
        &self.data
    }
}

impl Drawable for Buffer {
    fn width(&self) -> usize { self.width() }

    fn height(&self) -> usize { self.height() }
    
    fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.set_pixel(x, y, color); 
    }
}