pub struct Buffer {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        self.data.resize(new_width * new_height, 0);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.data.fill(color);
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.data
    }
}