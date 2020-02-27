use crate::graphics::primitive::quad;
use crate::core::rec::Rec;

pub mod primitive;
pub mod renderer;
pub mod target;
mod transformation;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Primitive {
    Quad {
        style: quad::Style,
        bound: Rec,
        scale: [f32; 2]
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RenderCommand {
    primitive: Primitive
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    quad_pipeline: quad::Pipeline,
}

impl Renderer {
    pub fn new(mut device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let quad_pipeline = quad::Pipeline::new(&mut device);

        Renderer {
            device,
            queue,
            quad_pipeline
        }
    }
    
    pub fn render(cmd: RenderCommand) {
        match cmd.primitive { 
            Primitive::Quad {
                style: style,
                bound: bound,
                scale: scale,
            } => {
                println!("render quad")
            }
        }
    }
}

#[test]
fn foo() {

    
}