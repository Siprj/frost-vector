use std::iter;

use wgpu::{
    include_wgsl, PipelineLayout, RenderPipeline, ShaderModule, BindGroupLayout, BindGroup,
};

use crate::{
    render_common::{RenderBase, PreparedRenderBase},
    windowed_device::WindowedDevice,
};

pub struct RendererTestTriangle {}

struct PreparedRendererTestTriangle {
    #[allow(dead_code)]
    shader: ShaderModule,
    #[allow(dead_code)]
    pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,
}

impl RenderBase for RendererTestTriangle {
    fn prepare(
        &self,
        windowed_device: &mut WindowedDevice,
        _projection_bind_group_layout: &BindGroupLayout,
    ) -> Box<dyn PreparedRenderBase> {
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

        Box::new(PreparedRendererTestTriangle {
            shader,
            pipeline_layout,
            render_pipeline,
        })
    }
}

impl PreparedRenderBase for PreparedRendererTestTriangle {
    fn render(&mut self, windowed_device: &mut WindowedDevice, _perspective_bind_group: &BindGroup) {
        let (mut encoder, view, surface) = windowed_device.prepare_encoder().unwrap();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        windowed_device.queue.submit(iter::once(encoder.finish()));
        surface.present();
    }
}
