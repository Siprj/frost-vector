use crate::raw::{Gpu, Raw};
use crate::render_common::{PreparedRenderBase, RenderBase};
use crate::statistics;
use crate::windowed_device::WindowedDevice;
use crate::{math, texture};
use log::debug;
use std::time::Instant;
use std::vec::Vec;
use std::{iter, mem};
use wgpu::util::DeviceExt;
use wgpu::{
    include_wgsl, BindGroup, BindGroupLayout, Extent3d, StoreOp, TextureDescriptor,
    TextureViewDescriptor,
};

#[derive(Debug, PartialEq, Clone)]
#[repr(C, packed)]
struct Circle {
    pos: math::Vector2<f32>, // Center position
    radius: f32,
    brush_size: f32,
}

impl Gpu for Circle {}

impl Circle {
    fn buffer_description<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Circle>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<math::Vector2<f32>>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<math::Vector2<f32>>() + mem::size_of::<f32>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

struct Vertex {
    #[allow(dead_code)]
    pos: math::Vector2<f32>,
    #[allow(dead_code)]
    uv_coords: math::Vector2<f32>,
}

impl Gpu for Vertex {}

impl Vertex {
    fn buffer_description<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<math::Vector2<f32>>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const CIRCLE_VERTICES: &[Vertex] = &[
    Vertex {
        pos: math::Vector2 { x: -1.0, y: 1.0 },
        uv_coords: math::Vector2 { x: -1.0, y: 1.0 },
    },
    Vertex {
        pos: math::Vector2 { x: 1.0, y: 1.0 },
        uv_coords: math::Vector2 { x: 1.0, y: 1.0 },
    },
    Vertex {
        pos: math::Vector2 { x: -1.0, y: -1.0 },
        uv_coords: math::Vector2 { x: -1.0, y: -1.0 },
    },
    Vertex {
        pos: math::Vector2 { x: 1.0, y: -1.0 },
        uv_coords: math::Vector2 { x: 1.0, y: -1.0 },
    },
];

const CIRCLE_INDICES: &[u16] = &[0, 1, 3, 3, 2, 0];

struct Renderer1Prepared {
    circles: Vec<Circle>,
    circle_vertex_buffer: wgpu::Buffer,
    circle_index_buffer: wgpu::Buffer,
    circle_pipeline: wgpu::RenderPipeline,
    circle_instances_buffer: wgpu::Buffer,
}

#[derive(Debug, Default)]
pub struct Renderer1 {
    circles: Vec<Circle>,
}

impl Renderer1 {
    pub fn circles(&mut self, circles: &Vec<(f32, f32, f32, f32)>) {
        for &(x, y, radius, brush_size) in circles {
            self.circle(x, y, radius, brush_size);
        }
    }
    pub fn circle(&mut self, x: f32, y: f32, radius: f32, brush_size: f32) {
        self.circles.push(Circle {
            pos: math::Vector2 { x, y },
            radius,
            brush_size,
        });
    }
}

impl RenderBase for Renderer1 {
    fn prepare(
        &self,
        windowed_device: &mut WindowedDevice,
        projection_bind_group_layout: &BindGroupLayout,
    ) -> Box<dyn PreparedRenderBase> {
        let circle_shader = windowed_device
            .device
            .create_shader_module(include_wgsl!("shaders/renderer_1_circle.wgsl"));

        let size = windowed_device.window.inner_size();
        math::ortho(0.0, size.width as f32, 0.0, size.height as f32, 0.0, 1.0);
        let render_pipeline_layout =
            windowed_device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[projection_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let circle_pipeline =
            windowed_device
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Circle Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &circle_shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::buffer_description(), Circle::buffer_description()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &circle_shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: windowed_device.config.format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Cw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                        // or Features::POLYGON_MODE_POINT
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 4,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    // If the pipeline will be used with a multiview render pass, this
                    // indicates how many array layers the attachments will have.
                    multiview: None,
                });

        let circle_vertex_buffer =
            windowed_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Vertex Buffer"),
                    contents: CIRCLE_VERTICES.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let circle_index_buffer =
            windowed_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Index Buffer"),
                    contents: CIRCLE_INDICES.get_raw(),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let circle_instances_buffer =
            windowed_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Index Buffer"),
                    contents: self.circles.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

        Box::new(Renderer1Prepared {
            circle_pipeline,
            circle_vertex_buffer,
            circle_index_buffer,
            circle_instances_buffer,
            circles: self.circles.clone(),
        })
    }
}

impl PreparedRenderBase for Renderer1Prepared {
    fn render(&mut self, windowed_device: &mut WindowedDevice, perspective_bind_group: &BindGroup) {
        let multi_sample_texture = windowed_device.device.create_texture(&TextureDescriptor {
            label: Some(&"msaa texture"),
            size: Extent3d {
                width: windowed_device.config.width,
                height: windowed_device.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: windowed_device.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[windowed_device.config.format],
        });

        let multi_sample_view = multi_sample_texture.create_view(&TextureViewDescriptor {
            label: Some(&"msaa view"),
            format: Some(windowed_device.config.format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: Some(1),
        });
        let (mut encoder, view, output) = windowed_device.prepare_encoder().unwrap();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Rectangle Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &multi_sample_view,
                    resolve_target: Some(&view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.render_circles(
                &mut render_pass,
                &self.circle_instances_buffer,
                perspective_bind_group,
            )
            .unwrap();
        }

        windowed_device.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}

impl Renderer1Prepared {
    fn render_circles<'a, 'b, 'c, 'd>(
        &'c self,
        render_pass: &'a mut wgpu::RenderPass<'d>,
        circle_instances_buffer: &'b wgpu::Buffer,
        perspective_bind_group: &'d BindGroup,
    ) -> Result<(), wgpu::SurfaceError>
    where
        'b: 'a,
        'c: 'a,
        'c: 'b,
        'b: 'd,
    {
        render_pass.set_pipeline(&self.circle_pipeline);
        render_pass.set_bind_group(0, perspective_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.circle_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, circle_instances_buffer.slice(..));
        render_pass.set_index_buffer(
            self.circle_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(
            0..(CIRCLE_INDICES.len() as u32),
            0,
            0..(self.circles.len() as u32),
        );
        Ok(())
    }
}
