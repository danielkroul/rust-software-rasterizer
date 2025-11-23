use crate::drawable::Drawable;

/* --- Bresenham's line algorithm ---
Parts of this algorithm seem entirely unintuitive.
It troubles me. I will add an explanation here, 
as soon as I understand it fully.
*/

/// Draws a line to any surface implementing the `Drawable` trait 
/// using Bresenham's line algorithm.
pub fn draw_line<D: Drawable>(
    surface: &mut D, 
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: u32,
) {
    let w = surface.width() as i32;
    let h = surface.height() as i32;

    // Early exit for lines fully outside screen or zero-length lines.
    if (x1 < 0 && x2 < 0) || (x1 >= w && x2 >= w) { return; }
    if (y1 < 0 && y2 < 0) || (y1 >= h && y2 >= h) { return; }
    if x1 == x2 && y1 == y2 {
        if x1 > 0 && y1 > 0 { 
            surface.set_pixel(x1 as usize, y1 as usize, color) 
        };
        return;
    }

    // Calculate deltas (the change on each axis)
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    // Determine the direction to step in x and y
    let step_x = if x2 > x1 { 1 } else { -1 };
    let step_y = if y2 > y1 { 1 } else { -1 };

    let y_major = dy > dx; // Check if y is driving axis

    let mut x = x1;
    let mut y = y1;

    // Swap deltas and step directions if y is driving axis
    let (dx, dy) = if y_major { (dy, dx) } else { (dx, dy) };
    let (step_x, step_y) = if y_major { (step_y, step_x) } else { (step_x, step_y) };

    // Helper closure for plotting pixels with bounds check
    let mut plot = |x: i32, y: i32| {
        if x >= 0 && y >= 0 && x < w && y < h {
            if y_major {
                surface.set_pixel(y as usize, x as usize, color);
            } else {
                surface.set_pixel(x as usize, y as usize, color);
            }
        }
    };

    let mut error = 2 * dy - dx; // Initialize error

    for _ in 0..=dx {
        plot(x, y);
        if error >= 0 {
            y += step_y;      // Move y when error passes threshold
            error -= 2 * dx;  // Correct error after moving y
        }
        error += 2 * dy;      // Increment error every step
        x += step_x;          // Always move along driving axis
    }
}