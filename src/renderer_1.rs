use crate::math;
use crate::raw::{Gpu, Raw};
use crate::render_common::{RenderBase, PreparedRenderBase};
use crate::statistics;
use crate::windowed_device::WindowedDevice;
use wgpu::{include_wgsl, BindGroupLayout, BindGroup, StoreOp};
use std::time::Instant;
use std::vec::Vec;
use std::{iter, mem};
use wgpu::util::DeviceExt;

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

#[derive(Clone, Debug)]
#[repr(C, packed)]
struct Rectangle {
    #[allow(unused)]
    pos: math::Vector2<f32>,
    #[allow(unused)]
    w: f32,
    #[allow(unused)]
    h: f32,
    #[allow(unused)]
    brush_size: f32,
}

impl Rectangle {
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<math::Vector2<f32>>()
                        + mem::size_of::<math::Vector2<f32>>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

impl Gpu for Rectangle {}

const RECTANGLE_VERTICES: &[Vertex] = &[
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

const RECTANGLE_INDICES: &[u16] = &[0, 1, 3, 3, 2, 0];

struct Renderer1Prepared {
    circles: Vec<Circle>,
    rectangles: Vec<Rectangle>,
    circle_vertex_buffer: wgpu::Buffer,
    circle_index_buffer: wgpu::Buffer,
    circle_pipeline: wgpu::RenderPipeline,
    rectangle_vertex_buffer: wgpu::Buffer,
    rectangle_index_buffer: wgpu::Buffer,
    rectangle_pipeline: wgpu::RenderPipeline,
    circle_instances_buffer: wgpu::Buffer,
    rectangle_instances_buffer: wgpu::Buffer,
}

#[derive(Debug, Default)]
pub struct Renderer1 {
    circles: Vec<Circle>,
    rectangles: Vec<Rectangle>,
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
    pub fn rectangles(&mut self, rectangles: &Vec<(f32, f32, f32, f32, f32)>) {
        for &(x, y, w, h, brush_size) in rectangles {
            self.rectangle(x, y ,w, h, brush_size);
        }
    }
    pub fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, brush_size: f32) {
        self.rectangles.push(Rectangle {
            pos: math::Vector2 { x, y },
            w,
            h,
            brush_size,
        });
    }
}

impl RenderBase for Renderer1 {
    fn prepare(&self, windowed_device: &mut WindowedDevice, projection_bind_group_layout: &BindGroupLayout) -> Box<dyn PreparedRenderBase> {
        let circle_shader = windowed_device
            .device
            .create_shader_module(include_wgsl!("shaders/renderer_1_circle.wgsl"));

        let rectangle_shader = windowed_device
            .device
            .create_shader_module(include_wgsl!("shaders/renderer_1_rectangle.wgsl"));

        let size = windowed_device.window.inner_size();
            math::ortho(0.0, size.width as f32, 0.0, size.height as f32, 0.0, 1.0);
        let render_pipeline_layout =
            windowed_device.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[projection_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let circle_pipeline = windowed_device
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
                    front_face: wgpu::FrontFace::Ccw,
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
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
            });

        let circle_vertex_buffer =
            windowed_device.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Vertex Buffer"),
                    contents: CIRCLE_VERTICES.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let circle_index_buffer = windowed_device
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Circle Index Buffer"),
                contents: CIRCLE_INDICES.get_raw(),
                usage: wgpu::BufferUsages::INDEX,
            });

        let rectangle_pipeline =
            windowed_device.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Rectangle Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &rectangle_shader,
                        entry_point: "vs_main",
                        buffers: &[
                            Vertex::buffer_description(),
                            Rectangle::buffer_description(),
                        ],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &rectangle_shader,
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
                        front_face: wgpu::FrontFace::Ccw,
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
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    // If the pipeline will be used with a multiview render pass, this
                    // indicates how many array layers the attachments will have.
                    multiview: None,
                });

        let rectangle_vertex_buffer =
            windowed_device.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("rectangle Vertex Buffer"),
                    contents: RECTANGLE_VERTICES.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let rectangle_index_buffer =
            windowed_device.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("rectangle Index Buffer"),
                    contents: RECTANGLE_INDICES.get_raw(),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let circle_instances_buffer = windowed_device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Circle Index Buffer"),
            size: 100000 * wgpu::COPY_BUFFER_ALIGNMENT,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let rectangle_instances_buffer = windowed_device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rectangle Index Buffer"),
            size: 100000 * wgpu::COPY_BUFFER_ALIGNMENT,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Box::new(Renderer1Prepared {
            circle_pipeline,
            circle_vertex_buffer,
            circle_index_buffer,
            rectangle_vertex_buffer,
            rectangle_index_buffer,
            rectangle_pipeline,
            circle_instances_buffer,
            rectangle_instances_buffer,
            circles: self.circles.clone(),
            rectangles: self.rectangles.clone(),
        })
    }
}

impl PreparedRenderBase for Renderer1Prepared {
    fn render(&mut self, windowed_device: &mut WindowedDevice, perspective_bind_group: &BindGroup) {
        if self.circles.get_raw().len()
            > self.circle_instances_buffer.size() as usize
        {
            let monotonic_time = Instant::now();
            let start = monotonic_time.elapsed();
            self.circle_instances_buffer =
                windowed_device
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Circle Index Buffer"),
                        contents: self.circles.get_raw(),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
            let end = monotonic_time.elapsed();
            statistics::report_value_with_name("bad_circle_path", (end - start).as_secs_f64());
        } else {
            let monotonic_time = Instant::now();
            let start = monotonic_time.elapsed();
            windowed_device.queue.write_buffer(
                &self.circle_instances_buffer,
                0,
                self.circles.get_raw(),
            );
            let end = monotonic_time.elapsed();
            statistics::report_value_with_name("good_circle_path", (end - start).as_secs_f64());
        }
        statistics::report_value_with_name(
            "circle_data_size",
            self.circles.get_raw().len() as f64,
        );
        statistics::report_value_with_name(
            "rectangle_data_size",
            self.rectangles.get_raw().len() as f64,
        );

        if self.rectangles.get_raw().len()
            > self.rectangle_instances_buffer.size() as usize
        {
            let monotonic_time = Instant::now();
            let start = monotonic_time.elapsed();
            self.rectangle_instances_buffer =
                windowed_device
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("rectangle Index Buffer"),
                        contents: self.rectangles.get_raw(),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
            let end = monotonic_time.elapsed();
            statistics::report_value_with_name(
                "bad_rectangle_path",
                (end - start).as_secs_f64(),
            );
        } else {
            let monotonic_time = Instant::now();
            let start = monotonic_time.elapsed();
            windowed_device.queue.write_buffer(
                &self.rectangle_instances_buffer,
                0,
                self.rectangles.get_raw(),
            );
            let end = monotonic_time.elapsed();
            statistics::report_value_with_name(
                "good_rectangle_path",
                (end - start).as_secs_f64(),
            );
        }

        let (mut encoder, view, output) = windowed_device.prepare_encoder().unwrap();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Rectangle Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })], depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None, });

            self.render_circles(&mut render_pass, &self.circle_instances_buffer, perspective_bind_group).unwrap();

            self.render_rectangles(&mut render_pass, &self.rectangle_instances_buffer, perspective_bind_group).unwrap();
        }

        {
            let monotonic_time = Instant::now();
            let start = monotonic_time.elapsed();
            windowed_device.queue.on_submitted_work_done(move || {
                statistics::report_value_with_name(
                    "end_queue_submit_time",
                    monotonic_time.elapsed().as_secs_f64(),
                )
            });
            windowed_device
                .queue
                .submit(iter::once(encoder.finish()));
            let end = monotonic_time.elapsed();
            statistics::report_value_with_name("queue_submit", (end - start).as_secs_f64());
            statistics::report_value_with_name("start_queue_submit_time", start.as_secs_f64());
        }

        {
            let monotonic_time = Instant::now();
            let start = monotonic_time.elapsed();
            output.present();
            let end = monotonic_time.elapsed();
            statistics::report_value_with_name("output_present", (end - start).as_secs_f64());
        }
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

    fn render_rectangles<'a, 'b, 'c, 'd>(
        &'c self,
        render_pass: &'a mut wgpu::RenderPass<'d>,
        rectangle_instances_buffer: &'b wgpu::Buffer,
        perspective_bind_group: &'d BindGroup,
    ) -> Result<(), wgpu::SurfaceError>
    where
        'b: 'a,
        'c: 'a,
        'c: 'b,
        'b: 'd,
    {
        render_pass.set_pipeline(&self.rectangle_pipeline);
        render_pass.set_bind_group(0, perspective_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.rectangle_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, rectangle_instances_buffer.slice(..));
        render_pass.set_index_buffer(
            self.rectangle_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(
            0..(RECTANGLE_INDICES.len() as u32),
            0,
            0..(self.rectangles.len() as u32),
        );
        Ok(())
    }

}
