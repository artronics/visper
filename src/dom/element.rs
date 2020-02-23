use crate::graphics::renderer::Renderable;

pub struct Element {
}

impl Renderable for Element {
    fn render() {
        println!("rendering element")
    }
}