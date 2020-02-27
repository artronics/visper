use wgpu::BackendBit;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};
use visper::visper::Visper;
use visper::graphics::Renderer;
use visper::graphics::target::Target;

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
    let size = window.inner_size().into();

    let renderer = Renderer::new(device, queue);
    let target = Target::new(&window, size, &renderer);

    let visper = Visper::new(renderer, target);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),

            event::Event::RedrawRequested(_) => {
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event::Event::WindowEvent {
                event: event::WindowEvent::Resized(size),
                ..
            } => {
                // sc_desc.width = size.width;
                // sc_desc.height = size.height;
                // swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            _ => {}
        }
    });
}