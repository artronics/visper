use crate::graphics::target::Target;
use wgpu::BackendBit;
use crate::graphics::Renderer;

pub struct Visper {
    renderer: Renderer,
    target: Target,
}

impl Visper {
    pub fn new(renderer: Renderer, target: Target) -> Self {
        Visper {
            renderer,
            target,
        }
    }
}