use winit::window;
use std::sync::Arc;

pub(crate) struct System {
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) is_surface_configured: bool,
    pub(crate) window: Arc<window::Window>,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
}

impl System {
    pub(crate) async fn new(window: Arc<window::Window>) -> anyhow::Result<Self> {
        // Create wgpu instance used to create adapter and surface
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            ..Default::default()
        });

        // Create surface to draw to using the window Arc pointer, allowing wgpu to talk to winit
        let surface = instance.create_surface(window.clone()).unwrap();

        // Request an adapter which can interface with the surface and GPU// Create adapter to interface with GPU hardware
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Create device to use GPU resources and queue to submit commands to GPU
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        // Get size of window
        let size = window.inner_size();

        // Get the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);
        // Loop through the available formats for the surface and find one that is sRGB
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        // Create surface configuration using the surface format and size of window as well as
        // Fifo present mode which is the only one that is guaranteed to be supported (also allows v-sync)
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            size,
        })
    }
}