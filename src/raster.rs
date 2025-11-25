use std::mem::swap;

use crate::drawable::Drawable;

/* --- Bresenham's line algorithm ---
Draws a line in integer coordinates using only integer addition, subtraction and comparisons.
Other strategies might use division and floating-point maths, which are computationally expensive.
The key idea:
- Track an integer "error" representing the distance between the discrete y and the ideal line.
- At each step in the driving axis (x for shallow lines), we increment the error by 2*dy.
- When the error passes the threshold, step in the other axis (y) and subtract 2*dx.
- This ensures exactly dy y-steps occur over dx x-steps, approximating the slope perfectly.
*/

/// Iterates over the integer coordinates of a line between two points using 
/// Bresenham's algorithm.
///
/// This function calls the provided `callback` closure for every 
/// point `(x, y)` on the line.
///
/// The line path is inclusive, meaning the callback is invoked for both the 
/// start (`x1`, `y1`) and end (`x2`, `y2`) coordinates.
fn walk_line<F: FnMut(i32, i32)>(
    mut x1: i32, mut y1: i32,
    x2: i32, y2: i32,
    mut callback: F,
) {
    if x1 == x2 && y1 == y2 {
        callback(x1, y1);
        return;
    }

    // Calculate deltas (the change on each axis)
    let mut dx = (x2 - x1).abs();
    let mut dy = (y2 - y1).abs();

    // Determine the direction to step in x and y
    let mut step_x = (x2 - x1).signum();
    let mut step_y = (y2 - y1).signum();

    let steep = dy > dx; // Check if y is driving axis

    // Swap axes if line is steep
    if steep {
        swap(&mut x1, &mut y1);
        swap(&mut dx, &mut dy);
        swap(&mut step_x, &mut step_y);
    }
    
    let err_dec = 2 * dx;
    let err_inc = 2 * dy;
    
    let (mut x, mut y) = (x1, y1);
    
    // --- Bresenham's Algorithm ---
    let mut err = err_inc - dx; // Initialize error
    
    for _ in 0..=dx {
        if steep { callback(y, x) } else { callback(x, y) }
        if err >= 0 {
            y += step_y; // Move y when error passes threshold
            err -= err_dec; // Correct error after moving y
        }
        err += err_inc; // Increment error every step
        x += step_x; // Always move along driving axis
    }
}

/// Draws a line from (`x1`, `y1`) to (`x2`, `y2`) to any surface 
/// implementing the `Drawable` trait.
pub fn draw_line<D: Drawable>(
    surface: &mut D, 
    x1: i32, y1: i32,
    x2: i32, y2: i32,
    color: u32,
) {
    let w = surface.width() as i32;
    let h = surface.height() as i32;

    /*
    TODO: Implement proper line-clipping here (e.g. Cohen-Sutherland).
    clip_line(&mut x1, &mut y1, &mut x2, &mut y2, w, h);
    If returns false (line rejected), return;

    After clipping, we know coordinates are safe. 
    We can use an unsafe plot or direct buffer access for speed.
    */

    // Helper closure for plotting pixels with bounds check
    let mut plot = |x: i32, y: i32| {
        if x >= 0 && y >= 0 && x < w && y < h {
            surface.set_pixel(x as usize, y as usize, color);
        }
    };

    // Early exit for zero-length lines.
    if x1 == x2 && y1 == y2 {
        plot(x1, y1);
        return;
    }
    
    // Calculate deltas (the change on each axis)
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    // Use helper functions for horizontal and vertical lines.
    if dy == 0 { 
        draw_horizontal(y1, x1, x2, w, h, &mut plot);
        return;
    }
    if dx == 0 { 
        draw_vertical(x1, y1, y2, w, h, &mut plot);
        return;
    }
    
    // Early exit for lines fully outside `surface`
    if (x1 < 0 && x2 < 0) || (x1 >= w && x2 >= w) { return; }
    if (y1 < 0 && y2 < 0) || (y1 >= h && y2 >= h) { return; }

    // Use helper function for diagonal lines.
    if dx == dy {
        draw_diagonal(x1, y1, x2, y2, &mut plot);
        return;
    }

    walk_line(x1, y1, x2, y2, |x, y| plot(x, y));
}

/// Private helper function for `draw_line`.
/// 
/// Draws a horizontal line from (`x1`, `y`) to (`x2`, `y`) to any surface
/// implementing the `Drawable` trait.
fn draw_horizontal(
    y: i32, 
    mut x1: i32, mut x2: i32, 
    w: i32, h: i32,
    plot: &mut impl FnMut(i32, i32),
) {
    // Horizontal line specific bounds checks and early exit
    if y < 0 || y >= h as i32 { return; }
    if (x1 < 0 && x2 < 0) || (x1 >= w && x2 >= w) { return; } 

    if x1 > x2 { swap(&mut x1, &mut x2); }
    (x1..=x2).for_each(|x| plot(x, y));
}

/// Private helper function for `draw_line`.
/// 
/// Draws a vertical line from (`x,` `y1`) to (`x,` `y2`) to any surface
/// implementing the `Drawable` trait.
fn draw_vertical(
    x: i32, 
    mut y1: i32, mut y2: i32, 
    w: i32, h: i32,
    plot: &mut impl FnMut(i32, i32),
) {
    // Vertical line specific bounds checks and early exit
    if x < 0 || x >= w as i32 { return; }
    if (y1 < 0 && y2 < 0) || (y1 >= h && y2 >= h) { return; } 

    if y1 > y2 { swap(&mut y1, &mut y2); }
    (y1..=y2).for_each(|y| plot(x, y));
}

/// Private helper function for `draw_line`.
/// 
/// Draws a diagonal line from (`x1`, `y1`) to (`x2`, `y2`) to any surface
/// implementing the `Drawable` trait.
fn draw_diagonal(
    x1: i32, y1: i32,
    x2: i32, y2: i32,
    plot: &mut impl FnMut(i32, i32),
) {
    // No extra bounds check here; uses bounds check from `draw_line`
    
    let step_x = (x2 - x1).signum();
    let step_y = (y2 - y1).signum();
    let steps = (x2 - x1).abs();

    let (mut x, mut y) = (x1, y1);

    for _ in 0..=steps {
        plot(x, y);
        x += step_x;
        y += step_y;
    }
}

/// Draws a filled triangle to any surface implementing the `Drawable` trait.
pub fn fill_triangle<D: Drawable>(surface: &mut D, v: &[(i32, i32)], color: u32) {
    // Determine bounding box of the triangle to know which Y-rows to clear/check
    let min_y = v[0].1.min(v[1].1).min(v[2].1);
    let max_y = v[0].1.max(v[1].1).max(v[2].1);
    
    // Create a "span buffer" 
    let height = (max_y - min_y + 1) as usize;
    let mut spans = vec![(i32::MAX, i32::MIN); height];

    // Define a closure that updates the spans directly
    let mut rasterize_edge = |x1, y1, x2, y2| {
        walk_line(x1, y1, x2, y2, |x, y| {
            if y >= min_y && y <= max_y {
                let idx = (y - min_y) as usize;
                if x < spans[idx].0 { spans[idx].0 = x; }
                if x > spans[idx].1 { spans[idx].1 = x; }
            }
        });
    };

    // Walk the edges
    rasterize_edge(v[0].0, v[0].1, v[1].0, v[1].1);
    rasterize_edge(v[1].0, v[1].1, v[2].0, v[2].1);
    rasterize_edge(v[2].0, v[2].1, v[0].0, v[0].1);

    // Draw the spans
    for (i, (start_x, end_x)) in spans.iter().enumerate() {
        if *start_x <= *end_x {
            let y = min_y + i as i32;
            draw_line(surface, *start_x, y, *end_x, y, color);
        }
    }
}