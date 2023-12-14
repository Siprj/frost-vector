use log::info;
use wgpu::{BindGroup, Buffer, util::DeviceExt, BindGroupLayout};
use winit::{event_loop::EventLoop, window::Window, event::Event, event::WindowEvent::{Resized, CloseRequested, KeyboardInput, MouseInput, RedrawRequested}, dpi::PhysicalSize};
use crate::{windowed_device::WindowedDevice, math, constants::NUMBER_OF_FRAMES, raw::Raw};

pub struct RendererRunner {
    wd: WindowedDevice,
    projection_bind_group: BindGroup,
    projection_buffer: Buffer,
    projection_bind_group_layout: BindGroupLayout,
    renderers: Vec<Box<dyn RenderBase>>,
}

impl RendererRunner {
    pub async fn new(renderers: Vec<Box<dyn RenderBase>>, event_loop: &mut EventLoop<()>) -> Self {
        let window = Window::new(&event_loop).unwrap();
        let mut wd = WindowedDevice::new(window).await;

        let (projection_buffer, projection_bind_group_layout, projection_bind_group) = Self::create_projection(&mut wd);
        Self {wd, projection_bind_group, projection_buffer, projection_bind_group_layout, renderers}
    }

    fn create_projection(wd: &mut WindowedDevice) -> (Buffer, BindGroupLayout, BindGroup){
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

        (projection_buffer, projection_bind_group_layout, projection_bind_group)
    }

    pub fn run(&mut self, event_loop: EventLoop<()>) {
        let mut render_count: u32 = 0;
        let current_renderer_base : Box<dyn RenderBase> = self.renderers.pop().expect("Renderer runner needs to be initialized with not enpty list of renderes!");
        info!("preparing the renderer instance");
        let mut current_renderer: Box<dyn PreparedRenderBase> = current_renderer_base.prepare(&mut self.wd, &self.projection_bind_group_layout);
        info!("preparation of the  the renderer instance is done");


        event_loop.run(move |event, elwt| {
            if let Event::WindowEvent{event, ..} = event {
                match event {
                    Resized(new_size) => {
                        self.update_projection(new_size);
                        elwt.exit()
                    },
                    CloseRequested => elwt.exit(),
                    KeyboardInput { device_id: _, event: _, is_synthetic: _ } => todo!(),
                    MouseInput { device_id: _, state: _, button: _ } => todo!(),
                    RedrawRequested => {
                        current_renderer.render(&mut self.wd, &self.projection_bind_group);
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
        }).unwrap();
    }
}

impl RendererRunner {
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


pub trait RenderBase {
    fn prepare(&self, windowed_device: &mut WindowedDevice, projection_bind_group_layout: &BindGroupLayout) -> Box<dyn PreparedRenderBase>;
}
pub trait PreparedRenderBase {
    fn render(&mut self, windowed_device: &mut WindowedDevice, perspective_bind_group: &BindGroup);
}
