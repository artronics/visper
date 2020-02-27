use crate::core::size::Size;
use winit::window::Window;
use crate::graphics::transformation::Transformation;
use crate::graphics::Renderer;

pub struct Target {
    surface: wgpu::Surface,
    size: Size,
    transformation: Transformation,
    swap_chain: wgpu::SwapChain,
}

impl Target {
    pub fn new(window: &Window, size: Size, renderer: &Renderer) -> Self {
        let (width, height) = size.into();
        let surface = wgpu::Surface::create(window);
        let swap_chain = new_swap_chain(&surface, width, height, &renderer.device);

        Target {
            surface,
            size,
            swap_chain,
            transformation: Transformation::orthographic(width, height),
        }
    }
}

fn new_swap_chain(
    surface: &wgpu::Surface,
    width: u16,
    height: u16,
    device: &wgpu::Device,
) -> wgpu::SwapChain {
    device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: u32::from(width),
            height: u32::from(height),
            present_mode: wgpu::PresentMode::Vsync,
        },
    )
}
