//use std::sync::Arc;
//use winit::window::Window;
use tiny_skia::Pixmap;
// Trait for drawable objects
pub trait Drawable {
    //fn draw(&self, buffer: &Arc<Window>);
    fn draw(&self, pixmap: &mut Pixmap);
}
