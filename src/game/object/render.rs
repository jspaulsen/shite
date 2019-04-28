// TODO(#3): Implement default Renderable
// This will provide a basic implementation for rendering objects
pub trait Renderable {
    fn render(&self /* camera/world */) {}
    fn should_render(&self) -> bool {
        false
    }
}
