use minifb::{Window, WindowOptions};
use rust_software_rasterizer::{buffer::Buffer, raster};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window = match Window::new(
        "Rust Software Rasterizer",
        WIDTH, 
        HEIGHT, 
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(err) => {
            eprintln!("Window creation failed: {}", err);
            return;
        }
    };

    let mut buffer = Buffer::new(WIDTH, HEIGHT);

    let vertices: Vec<(i32, i32)> = vec![
        (100, 100),
        (500, 200),
        (300, 500),
    ];

    while window.is_open() {

        buffer.clear(0);

        raster::fill_triangle(&mut buffer, vertices.as_slice(), 0xFF00FF);

        if let Err(e) = window.update_with_buffer(buffer.as_slice(), WIDTH, HEIGHT) {
            eprintln!("Warning: failed to update buffer: {}", e);
            continue;
        }
    }
}