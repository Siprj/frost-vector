use wgpu::{
    include_wgsl, PipelineLayout, RenderPipeline, ShaderModule,
};
use winit::{
    event::WindowEvent,
    event_loop::EventLoop,
};

use crate::{
    render_common::{PreparedRenderBase2, RenderBase2, RenderBase, PreparedRenderBase},
    windowed_device::WindowedDevice,
};

pub struct RendererTestTriangle {}

struct PreparedRendererTestTriangle {
    #[allow(dead_code)]
    shader: ShaderModule,
    #[allow(dead_code)]
    wd: WindowedDevice,
    #[allow(dead_code)]
    pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,
}

impl RenderBase2 for RendererTestTriangle {
    async fn prepare(&self, event_loop: &mut EventLoop<()>) -> Box<(dyn PreparedRenderBase2)> {
        let window = winit::window::Window::new(event_loop).unwrap();
        let wd = WindowedDevice::new(window).await;

        // Load the shaders from disk
        let shader = wd
            .device
            .create_shader_module(include_wgsl!("shaders/renderer_test_triangle.wgsl"));

        let pipeline_layout = wd
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = wd
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wd.config.format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Box::new(PreparedRendererTestTriangle {
            shader,
            wd,
            pipeline_layout,
            render_pipeline,
        })
    }
}

impl PreparedRenderBase2 for PreparedRendererTestTriangle {
    fn render(&mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, elwt| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.

            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Resized(size) => {
                            // Reconfigure the surface with the new size
                            self.wd.config.width = size.width;
                            self.wd.config.height = size.height;
                            self.wd.surface.configure(&self.wd.device, &self.wd.config);
                            // On macos the window needs to be redrawn manually after resizing
                            self.wd.window.request_redraw();
                        }
                        WindowEvent::RedrawRequested => {
                        }
                        WindowEvent::CloseRequested => elwt.exit(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }).unwrap();
    }
}

pub struct RendererTestTriangle2 {}

struct PreparedRendererTestTriangle2 {
    #[allow(dead_code)]
    shader: ShaderModule,
    #[allow(dead_code)]
    pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,
}

impl RenderBase for RendererTestTriangle2 {
    fn prepare(
        &self,
        windowed_device: &mut WindowedDevice,
        projection_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn crate::render_common::PreparedRenderBase> {
        // Load the shaders from disk
        let shader = windowed_device
            .device
            .create_shader_module(include_wgsl!("shaders/renderer_test_triangle.wgsl"));

        let pipeline_layout = windowed_device
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = windowed_device
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(windowed_device.config.format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Box::new(PreparedRendererTestTriangle2 {
            shader,
            pipeline_layout,
            render_pipeline,
        })
    }
}

impl PreparedRenderBase for PreparedRendererTestTriangle2 {
    fn render(&mut self, windowed_device: &mut WindowedDevice, _perspective_bind_group: &wgpu::BindGroup) {
        let (mut encoder, view, surface) = windowed_device.prepare_encoder().unwrap();
            {
                let mut rpass =
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                    store: wgpu::StoreOp::Store,
                                },
                            },
                        )],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                rpass.set_pipeline(&self.render_pipeline);
                rpass.draw(0..3, 0..1);
            }

            windowed_device.queue.submit(Some(encoder.finish()));
            surface.present();
    }
}
