pub mod main_pipeline;

use log::info;
use raw_window_handle::HasRawWindowHandle;

pub struct GpuInfo {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swapchain: wgpu::SwapChain,
}

impl GpuInfo {
    pub async fn new<W: HasRawWindowHandle>(window: &W, window_size: &na::Vector2<u32>) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
            })
            .await
            .expect("Failed to get a suitable render adapter");

        info!("Selected GPU: {}", adapter.get_info().name);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .await
            .expect("Failed to create render device");

        let swapchain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                width: window_size.x,
                height: window_size.y,
                format: adapter.get_swap_chain_preferred_format(&surface),
                present_mode: wgpu::PresentMode::Fifo,
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT | /* TODO: why? --> */ wgpu::TextureUsage::COPY_SRC,
            },
        );

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swapchain,
        }
    }
}
