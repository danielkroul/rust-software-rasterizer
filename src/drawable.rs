/// Anything that can be drawn to.
pub trait Drawable {
    /// Width of the drawable surface.
    fn width(&self) -> usize;
    /// Height of the drawable surface.
    fn height(&self) -> usize;
    /// Set a pixel at (x,y) to a given color.
    fn set_pixel(&mut self, x: usize, y: usize, color: u32);
}