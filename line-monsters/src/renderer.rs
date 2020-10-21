pub mod spritebatch;
pub mod texture;

use crate::camera::{Camera, CameraController};
use std::sync::Arc;
use ultraviolet::{Mat4, Vec3};
use winit::{event::WindowEvent, window::Window};

pub struct State {
    surface: wgpu::Surface,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,

    #[allow(dead_code)]
    camera: Camera,

    #[allow(dead_code)]
    uniforms: Uniforms,
    #[allow(dead_code)]
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    camera_controller: CameraController,

    depth_texture: Arc<texture::Texture>,

    #[allow(dead_code)]
    render_texture_depth_texture: Arc<texture::Texture>,
    #[allow(dead_code)]
    render_texture: Arc<texture::Texture>,

    pub spritebatch_buffers: Vec<(
        Arc<texture::Texture>,
        spritebatch::VertexBuffer,
        spritebatch::IndexBuffer,
    )>,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapater = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapater
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let device = Arc::new(device);

        let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: texture_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let diffuse_bind_group_layout = texture::Texture::create_bind_group_layout(&device);

        // Camera
        let eye = Vec3::new(0.5, 20.0, 20.5);
        let direction = Vec3::new(0., -1.0, -1.0);

        let camera = Camera {
            eye,
            direction,
            up: Vec3::unit_y(),
            aspect: 256. / 192.,
            fov_y: 25.0_f32.to_radians(),
            z_near: 0.01,
            z_far: 50.0,
        };

        // Uniforms

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        use wgpu::util::DeviceExt;
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform_buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("unform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        // Render pipeline
        let vs_module = device.create_shader_module(wgpu::include_spirv!("res/shader.vert.spv"));
        let fs_module = device.create_shader_module(wgpu::include_spirv!("res/shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&diffuse_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: texture_format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::descriptor()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let camera_controller = CameraController::new(0.2);

        let depth_texture = texture::Texture::create_depth_texture(
            &device,
            sc_desc.width,
            sc_desc.height,
            "depth_texture",
        );

        let render_texture_depth_texture =
            texture::Texture::create_depth_texture(&device, 256, 192, "rt_depth_texture");
        let render_texture =
            texture::Texture::empty_texture(&device, &queue, 256, 192, "render_Texture");

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size: window_size,

            render_pipeline,

            camera,

            uniforms,
            uniform_buffer,
            uniform_bind_group,

            camera_controller,

            depth_texture,

            render_texture_depth_texture,
            render_texture,

            spritebatch_buffers: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        self.depth_texture = texture::Texture::create_depth_texture(
            &self.device,
            self.sc_desc.width,
            self.sc_desc.height,
            "depth_texture",
        );
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn render(&mut self, window: &Window) {
        let frame = self.swap_chain.get_current_frame();
        let frame = match frame {
            Result::Err(wgpu::SwapChainError::Outdated) => {
                self.resize(window.inner_size());
                self.swap_chain.get_current_frame().unwrap().output
            }
            Result::Ok(swap_chain) => swap_chain.output,
            _ => panic!("Timeout getting texture"),
        };

        let buffer_one = {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let mut bind_groups = Vec::new();
            for (texture, _, _) in self.spritebatch_buffers.iter() {
                let (diffuse_bind_group, _) = texture.create_bind_group(&self.device);
                bind_groups.push(diffuse_bind_group);
            }

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    //attachment: &self.render_texture.view,
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    //attachment: &self.render_texture_depth_texture.view,
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            for ((_, vertex_buffer, index_buffer), bind_group) in
                self.spritebatch_buffers.iter().zip(bind_groups.iter())
            {
                let spritebatch::IndexBuffer(index_buffer, num_indices) = index_buffer;
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.0.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..));
                render_pass.draw_indexed(0..*num_indices, 0, 0..1);
            }
            drop(render_pass);

            encoder.finish()
        };

        self.queue.submit(std::iter::once(buffer_one));
        /*let buffer_two = {
            #[rustfmt::skip]
            const VERTICES: &[Vertex] = &[
                Vertex { position: [-1., 1., 0.], tex_coords: [0., 0.], }, // A
                Vertex { position: [-1., -1., 0.], tex_coords: [0., 1.], }, // B
                Vertex { position: [1., -1., 0.], tex_coords: [1., 1.], }, // C
                Vertex { position: [1., 1., 0.], tex_coords: [1., 0.], }, // D
            ];

            #[rustfmt::skip]
            const INDICES: &[u16] = &[
                0, 1, 3,
                1, 2, 3,
            ];

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let (bind_group, _) = self.render_texture.create_bind_group(&self.device);

            // Create buffers
            use wgpu::util::DeviceExt;
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&VERTICES),
                    usage: wgpu::BufferUsage::VERTEX,
                });

            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    #[rustfmt::skip]
                    contents: bytemuck::cast_slice(&INDICES),
                    usage: wgpu::BufferUsage::INDEX,
                });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            let uniform = Uniforms {
                view: Mat4::identity(),
                ortho_proj: Mat4::identity(),
                perspective_proj: Mat4::identity(),
                render_target: true,
            };

            //self.queue
            //    .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..));
            render_pass.draw_indexed(0..6, 0, 0..1);
            drop(render_pass);

            encoder.finish()
        };

        self.queue.submit(std::iter::once(buffer_two));*/
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    view: Mat4,
    ortho_proj: Mat4,
}

impl Uniforms {
    fn new() -> Self {
        Self {
            view: Mat4::identity(),
            ortho_proj: Mat4::identity(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        let (view, ortho_proj) = camera.build_view_projection_matrix();
        self.view = view;
        self.ortho_proj = ortho_proj;
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}
