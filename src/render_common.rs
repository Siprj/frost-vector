use wgpu::{BindGroup, Buffer, util::DeviceExt};
use winit::{event_loop::EventLoop, window::Window, event::Event, event::WindowEvent::{Resized, CloseRequested, KeyboardInput, MouseInput}, dpi::PhysicalSize};
use crate::{windowed_device::WindowedDevice, math, constants::NUMBER_OF_FRAMES, raw::Raw};


struct RendererRunner {
    wd: WindowedDevice,
    projection_bind_group: BindGroup,
    projection_buffer: Buffer,
    event_loop: EventLoop<()>,
    renderers: Vec<Box<dyn RenderBase>>,
}

impl RendererRunner {
    pub async fn new(renderers: Vec<Box<dyn RenderBase>>) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();
        let mut wd = WindowedDevice::new(window).await;

        let (projection_buffer, projection_bind_group) = Self::create_projection(&mut wd);
        Self { wd, projection_bind_group, projection_buffer, event_loop, renderers }
    }

    fn create_projection(wd: &mut WindowedDevice) -> (Buffer, BindGroup){
        let size = wd.window.inner_size();
        let perspective_matrix: math::Matrix4x4<f32> =
            math::ortho(0.0, size.width as f32, 0.0, size.height as f32, 0.0, 1.0);

        let projection_buffer = wd
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Projection Buffer"),
                contents: perspective_matrix.get_raw(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                // TODO: Check if the COPY_DST is needed.
            });

        let projection_bind_group_layout =
            wd.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Projection Bind Group Descriptor"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let projection_bind_group = wd.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: projection_buffer.as_entire_binding(),
            }],
            label: Some("Projection Bind Group"),
        });

        (projection_buffer, projection_bind_group)
    }

    fn run(&mut self) {
        let mut render_count: u32 = 0;
        let mut current_renderer: Box<dyn RenderBase> = self.renderers.pop().expect("rendereer runner needs to be initialized with not enpty list of renderes");

        self.event_loop.run(move |event, elwt| {
            if let Event::WindowEvent{event, ..} = event {
                match event {
                    Resized(new_size) => {
                        self.update_projection(new_size);
                        elwt.exit()
                    },
                    CloseRequested => elwt.exit(),
                    KeyboardInput { device_id, event, is_synthetic } => todo!(),
                    MouseInput { device_id, state, button } => todo!(),
                    RedrawRequested => {
                        render_count += 1;
                        if render_count > NUMBER_OF_FRAMES {
                            render_count = 0;
                            // TODO:
                            //  1. Publish statistics
                            //  2. Reset statistics
                            //  3. Switch to different renderer
                            todo!("");
                        }

                    }
                    _ => ()
                }
            }
        });
    }

    fn update_projection(&mut self, new_size: PhysicalSize<u32>) {
        let projection_matrix: math::Matrix4x4<f32> =
            math::ortho(0.0, new_size.width as f32, 0.0, new_size.height as f32, 0.0, 1.0);
        self.wd.queue.write_buffer(
            &self.projection_buffer,
            0,
            projection_matrix.get_raw(),
        );
    }
}


trait RenderBase {
    fn prepare(&mut self, windowed_device: &mut WindowedDevice);
    fn run(self, windowed_device: &mut WindowedDevice, perspectiveBindGroup: &BindGroup);
}
