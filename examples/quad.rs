use visper::graphics::primitive::quad::Pipeline;

use wgpu::BackendBit;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
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

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let p = Pipeline::new(&mut device);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),

            event::Event::RedrawRequested(_) => {
                let frame = swap_chain
                    .get_next_texture();
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::WHITE,
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&p.pipeline);
                    rpass.set_bind_group(0, &p.bind_group, &[]);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(&[encoder.finish()]);
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event::Event::WindowEvent {
                event: event::WindowEvent::Resized(size),
                ..
            } => {
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            _ => {}
        }
    });
}