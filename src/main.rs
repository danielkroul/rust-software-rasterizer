use minifb::{Window, WindowOptions};
use rust_software_rasterizer::{buffer::Buffer, raster};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window = match Window::new(
        "Rust Software Rasterizer",
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    ) {
        Ok(win) => win,
        Err(err) => {
            eprintln!("Window creation failed: {}", err);
            return;
        }
    };

    let mut buffer = Buffer::new(WIDTH, HEIGHT);
    buffer.clear(0x202015);
    buffer.set_pixel(50, 50, 0xFF0000);
    raster::draw_line(&mut buffer, 100, 100, 600, 50, 0xFFFF00);

    while window.is_open() {
        if let Err(e) = window.update_with_buffer(buffer.as_slice(), WIDTH, HEIGHT) {
            eprintln!("Warning: failed to update buffer: {}", e);
            continue;
        }
    }
}