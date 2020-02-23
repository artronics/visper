use crate::graphics::transformation::Transformation;


pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
}

impl Pipeline {
    pub fn new(device: &mut wgpu::Device) -> Self {
        let vs = include_bytes!("shaders/hello/v");
        let vs_module = device.create_shader_module(
            &wgpu::read_spirv(std::io::Cursor::new(&vs[..]))
                .expect("Read quad vertex shader as SPIR-V"),
        );

        let fs = include_bytes!("shaders/hello/f");
        let fs_module = device.create_shader_module(
            &wgpu::read_spirv(std::io::Cursor::new(&fs[..]))
                .expect("Read quad fragment shader as SPIR-V"),
        );

        let vertex_stage = wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        };

        let fragment_stage = wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        };

        let vertex_buffers = {
            &[
                // wgpu::VertexBufferDescriptor {
                //     stride: std::mem::size_of::<Vertex>() as u64,
                //     step_mode: wgpu::InputStepMode::Vertex,
                //     attributes: &[wgpu::VertexAttributeDescriptor {
                //         shader_location: 0,
                //         format: wgpu::VertexFormat::Float2,
                //         offset: 0,
                //     }],
                // }
            ]
        };

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage,
            fragment_stage: Some(fragment_stage),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Pipeline {
            pipeline,
            bind_group,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wgpu::BackendBit;
    use winit::{
        event,
        event_loop::{ControlFlow, EventLoop},
    };

    #[test]
    fn test_pipeline_render() {
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            backends: BackendBit::all(),
        }).unwrap();

        let (mut device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size();
        let surface = wgpu::Surface::create(&window);

        let p = Pipeline::new(&mut device);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    _position: [f32; 2],
}

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        _position: [0.0, 0.0],
    },
    Vertex {
        _position: [1.0, 0.0],
    },
    Vertex {
        _position: [1.0, 1.0],
    },
    Vertex {
        _position: [0.0, 1.0],
    },
];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
}

impl Quad {
    const MAX: usize = 100_000;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Uniforms {
    transform: [f32; 16],
    scale: f32,
}

impl Uniforms {
    fn new(transformation: Transformation, scale: f64) -> Uniforms {
        Self {
            transform: *transformation.as_ref(),
            scale: scale as f32,
        }
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: *Transformation::identity().as_ref(),
            scale: 1.0,
        }
    }
}

