use crate::math;
use crate::raw::{Gpu, Raw};
use crate::windowed_device::WindowedDevice;
use notify::{Error, RecursiveMode, Watcher};
use std::process::exit;
use std::sync::mpsc::{self, channel, Receiver};
use std::vec::Vec;
use std::{env, iter, mem};
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug, PartialEq)]
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

pub struct DrawObjects {
    circles: Vec<Circle>,
    rectangles: Vec<Rectangle>,
}

impl DrawObjects {
    fn new() -> Self {
        DrawObjects {
            circles: Vec::new(),
            rectangles: Vec::new(),
        }
    }
    pub fn circle(&mut self, x: f32, y: f32, radius: f32, brush_size: f32) {
        self.circles.push(Circle {
            pos: math::Vector2 { x, y },
            radius,
            brush_size,
        });
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

struct Renderer {
    pub windowed_device: WindowedDevice,
    pub drawable_objects: DrawObjects,
    pub circle_vertex_buffer: wgpu::Buffer,
    pub circle_index_buffer: wgpu::Buffer,
    pub circle_pipeline: wgpu::RenderPipeline,
    pub rectangle_vertex_buffer: wgpu::Buffer,
    pub rectangle_index_buffer: wgpu::Buffer,
    pub rectangle_pipeline: wgpu::RenderPipeline,
    pub perspective_bind_group: wgpu::BindGroup,
    pub perspective_buffer: wgpu::Buffer,
    pub file_event_receiver: Receiver<Result<notify::Event, Error>>,

    // We need to the norifier needs to exist as long as the renderer exists,
    // so we get the filesystem events.
    #[allow(unused)]
    watcher: notify::INotifyWatcher,
}

impl Renderer {
    pub async fn new(window: Window) -> Self {
        let wd = WindowedDevice::new(window).await;

        let circle_shader = wd
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("renderer_1_circle_shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("shaders/renderer_1_circle.wgsl").into(),
                ),
            });

        let rectangle_shader = wd
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("renderer_1_rectangle_shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("shaders/renderer_1_rectangle.wgsl").into(),
                ),
            });

        let size = wd.window.inner_size();
        let perspective_matrix: math::Matrix4x4<f32> =
            math::ortho(size.width as u16, size.height as u16);
        let perspective_buffer = wd
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Perspective Buffer"),
                contents: perspective_matrix.get_raw(),
                // contents: bytemuck::cast_slice(&perspective_matrix_bla),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let perspective_bind_group_layout =
            wd.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("perspective Bind Group Descriptor"),
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

        let perspective_bind_group = wd.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &perspective_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: perspective_buffer.as_entire_binding(),
            }],
            label: Some("Perspective Bind Group"),
        });

        let render_pipeline_layout =
            wd.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&perspective_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let circle_pipeline = wd
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
                        format: wd.config.format,
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
            wd.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Vertex Buffer"),
                    contents: CIRCLE_VERTICES.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        println!(
            "circle indexs size in bytes: {}",
            CIRCLE_INDICES.get_raw().len()
        );
        let circle_index_buffer = wd
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Circle Index Buffer"),
                contents: CIRCLE_INDICES.get_raw(),
                usage: wgpu::BufferUsages::INDEX,
            });

        let rectangle_pipeline =
            wd.device
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
                            format: wd.config.format,
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
            wd.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("rectangle Vertex Buffer"),
                    contents: RECTANGLE_VERTICES.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        println!(
            "rectangle indexs size in bytes: {}",
            RECTANGLE_INDICES.get_raw().len()
        );
        let rectangle_index_buffer =
            wd.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("rectangle Index Buffer"),
                    contents: RECTANGLE_INDICES.get_raw(),
                    usage: wgpu::BufferUsages::INDEX,
                });

        // Create a channel to receive the events.
        let (sender, receiver) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = notify::recommended_watcher(sender).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        let mut watched_path = env::current_dir().unwrap();
        watched_path.push("src/shaders");
        watcher
            .watch(&watched_path, RecursiveMode::Recursive)
            .unwrap();

        Self {
            windowed_device: wd,
            drawable_objects: DrawObjects::new(),
            circle_pipeline,
            circle_vertex_buffer,
            circle_index_buffer,
            rectangle_vertex_buffer,
            rectangle_index_buffer,
            rectangle_pipeline,
            perspective_bind_group,
            perspective_buffer,
            file_event_receiver: receiver,
            watcher,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let circle_instances_buffer =
            self.windowed_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Index Buffer"),
                    contents: self.drawable_objects.circles.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

        let rectangle_instances_buffer =
            self.windowed_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Rectangle Index Buffer"),
                    contents: self.drawable_objects.rectangles.get_raw(),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

        let (mut encoder, view, output) = self.windowed_device.prepare_encoder()?;
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
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.render_circles(&mut render_pass, &circle_instances_buffer)?;

            self.render_rectangles(&mut render_pass, &rectangle_instances_buffer)?;
        }

        self.windowed_device
            .queue
            .submit(iter::once(encoder.finish()));
        output.present();

        self.drawable_objects.circles.clear();
        self.drawable_objects.rectangles.clear();
        Ok(())
    }

    fn render_circles<'a, 'b, 'c, 'd>(
        &'c self,
        render_pass: &'a mut wgpu::RenderPass<'d>,
        circle_instances_buffer: &'b wgpu::Buffer,
    ) -> Result<(), wgpu::SurfaceError>
    where
        'b: 'a,
        'c: 'a,
        'c: 'b,
        'b: 'd,
    {
        render_pass.set_pipeline(&self.circle_pipeline);
        render_pass.set_bind_group(0, &self.perspective_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.circle_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, circle_instances_buffer.slice(..));
        render_pass.set_index_buffer(
            self.circle_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(
            0..(CIRCLE_INDICES.len() as u32),
            0,
            0..(self.drawable_objects.circles.len() as u32),
        );
        Ok(())
    }

    fn render_rectangles<'a, 'b, 'c, 'd>(
        &'c self,
        render_pass: &'a mut wgpu::RenderPass<'d>,
        rectangle_instances_buffer: &'b wgpu::Buffer,
    ) -> Result<(), wgpu::SurfaceError>
    where
        'b: 'a,
        'c: 'a,
        'c: 'b,
        'b: 'd,
    {
        render_pass.set_pipeline(&self.rectangle_pipeline);
        render_pass.set_bind_group(0, &self.perspective_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.rectangle_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, rectangle_instances_buffer.slice(..));
        render_pass.set_index_buffer(
            self.rectangle_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(
            0..(RECTANGLE_INDICES.len() as u32),
            0,
            0..(self.drawable_objects.rectangles.len() as u32),
        );
        Ok(())
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.windowed_device.config.width = new_size.width;
            self.windowed_device.config.height = new_size.height;
            self.windowed_device
                .surface
                .configure(&self.windowed_device.device, &self.windowed_device.config);

            let perspective_matrix: math::Matrix4x4<f32> =
                math::ortho(new_size.width as u16, new_size.height as u16);
            println!("perspective_matrix_bla: {:?}", perspective_matrix);
            self.windowed_device.queue.write_buffer(
                &self.perspective_buffer,
                0,
                bytemuck::cast_slice(perspective_matrix.get_raw()),
            );
        }
    }

    fn update_shaders_if_needed(&mut self) {
        match self.file_event_receiver.try_recv() {
            Ok(res) => {
                println!("nice stuff; file changed: {:?}", res);
                let perspective_bind_group_layout = self
                    .windowed_device
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("perspective Bind Group Descriptor"),
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
                let render_pipeline_layout = self.windowed_device.device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[&perspective_bind_group_layout],
                        push_constant_ranges: &[],
                    },
                );

                let mut circle_shader_path = env::current_dir().unwrap();
                circle_shader_path.push("src/shaders/renderer_1_circle.wgsl");
                let circle_shader_content = std::fs::read_to_string(circle_shader_path).unwrap();

                let circle_shader = self.windowed_device.device.create_shader_module(
                    wgpu::ShaderModuleDescriptor {
                        label: Some("renderer_1_circle_shader"),
                        source: wgpu::ShaderSource::Wgsl(circle_shader_content.into()),
                    },
                );

                let circle_pipeline = self.windowed_device.device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
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
                                format: self.windowed_device.config.format,
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
                    },
                );

                let mut rectangle_shader_path = env::current_dir().unwrap();
                rectangle_shader_path.push("src/shaders/renderer_1_rectangle.wgsl");
                let rectangle_shader_content =
                    std::fs::read_to_string(rectangle_shader_path).unwrap();

                let rectangle_shader = self.windowed_device.device.create_shader_module(
                    wgpu::ShaderModuleDescriptor {
                        label: Some("renderer_1_rectangle_shader"),
                        source: wgpu::ShaderSource::Wgsl(rectangle_shader_content.into()),
                    },
                );

                let rectangle_pipeline = self.windowed_device.device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
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
                                format: self.windowed_device.config.format,
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
                    },
                );
                self.rectangle_pipeline = rectangle_pipeline;
                self.circle_pipeline = circle_pipeline;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("file listeners is disconnected!!!");
                exit(1);
            }
        }
    }
}

pub async fn run<F, F2>(event_handler: F, mut redraw: F2)
where
    F: 'static + Fn(&WindowEvent<'_>) -> bool,
    F2: 'static + FnMut(&mut DrawObjects),
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut renderer = Renderer::new(window).await;
    event_loop.run(move |event, _, control_flow| {
        renderer.update_shaders_if_needed();
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.windowed_device.window.id() => {
                if !event_handler(&event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            renderer.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id)
                if window_id == renderer.windowed_device.window.id() =>
            {
                redraw(&mut renderer.drawable_objects);
                match renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        panic!("wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated");
                        //self.resize(self.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                renderer.windowed_device.window.request_redraw();
            }
            _ => {}
        }
    });
}
