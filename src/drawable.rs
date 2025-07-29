// Trait for drawable objects
pub trait Drawable {
    fn draw(&self, pixmap: &mut Pixmap);
}
