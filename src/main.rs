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

    let (x1, y1) = (300, 300);
    let (mut x2, mut y2) = (200, 250);

    while window.is_open() {
        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            x2 = mx as i32;
            y2 = my as i32;
        }

        buffer.clear(0);
        buffer.set_pixel(50, 50, 0xFF0000);

        raster::draw_line(&mut buffer, x1, y1, x2, y2, 0xFFFF00);
        buffer.set_pixel(x1 as usize, y1 as usize, 0x00FF00);
        buffer.set_pixel(x2 as usize, y2 as usize, 0xFF0000);

        if let Err(e) = window.update_with_buffer(buffer.as_slice(), WIDTH, HEIGHT) {
            eprintln!("Warning: failed to update buffer: {}", e);
            continue;
        }
    }
}