use log::info;
use wgpu::Adapter;
use winit::window::Window;

// TODO: Try to think of a better name.
pub struct WindowedDevice {
    pub instance: wgpu::Instance,
    // Configuration of the droweable surface. It is here so we can reconfigure
    // resolution on resize event.
    pub config: wgpu::SurfaceConfiguration,
    // Logical representaion of the GPU device crates encoders and it used to
    // reconfigure the window.
    pub device: wgpu::Device,
    // Sends data and encoded commands to GPU
    pub queue: wgpu::Queue,
    // Drawable surface in the window.
    pub surface: wgpu::Surface,
    // Represents the system window.
    pub window: Window,
    pub adapter: Adapter,
}

impl WindowedDevice {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // Default is LowPower what is why I change it.
                // power_preference: wgpu::PowerPreference::default(),
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GPU device"),
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let swap_chain_capablities = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        println!("surface formats: {:?}", swap_chain_capablities.formats);
        let swapchain_format = swap_chain_capablities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(swap_chain_capablities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            // vsync off
            // present_mode: wgpu::PresentMode::AutoNoVsync, // surface_caps.present_modes[0],
            present_mode: wgpu::PresentMode::AutoVsync, // surface_caps.present_modes[0],
            alpha_mode: swap_chain_capablities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            window,
            adapter,
        }
    }

    // TODO: Is it possible to do this only onece???
    // * Looks like encoder could be put into "global" state.
    // output and view seams to be part of the swapchain so I guess they need to be
    // recreated every time???
    pub fn prepare_encoder(
        &mut self,
    ) -> Result<
        (
            wgpu::CommandEncoder,
            wgpu::TextureView,
            wgpu::SurfaceTexture,
        ),
        wgpu::SurfaceError,
    > {
        info!("getting current surface texture");
        let output = /*match */ self.surface.get_current_texture()?; //{
                                                                     //            Ok(output) => output,
                                                                     //            Err(err) => match err {
                                                                     //                wgpu::SurfaceError::Timeout => todo!(),
                                                                     //                wgpu::SurfaceError::Outdated => todo!(),
                                                                     //                wgpu::SurfaceError::Lost => {},
                                                                     //                wgpu::SurfaceError::OutOfMemory => todo!(),
                                                                     //            },
                                                                     //        };
        info!("creating view from the texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        info!("getting commander encoder");
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("GPU Encoder"),
            });
        Ok((encoder, view, output))
    }
}
