pub mod texture;

use bytemuck::{Pod, Zeroable};
use wgpu::{include_wgsl, util::DeviceExt, Backends, Instance};
use winit::{dpi::PhysicalSize, window::Window};

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 2, 3,
    0, 1, 2
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Pan {
    pub width: i32,
    pub height: i32,
}

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub pan: Pan,
    pub scale: f32,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    num_indices: u32,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    texture: texture::Texture,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Pan {
    pub fn increase_width(&mut self, v: i32) {
        self.width += v;
    }
    pub fn increase_height(&mut self, v: i32) {
        self.height += v;
    }
}

impl Renderer {
    pub async fn new(window: &Window, img: &texture::Rgba8Image) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER,
                    limits: Default::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let texture = texture::Texture::from_image(&device, &queue, img, Some("yume texture"));

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("yume texture bind group layout"),
            });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("yume diffuse bind group"),
        });

        let shader = device.create_shader_module(&include_wgsl!("../../../shaders/shader.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("yume pipeline layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("yume pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("yume vertex buffer"),
            contents: bytemuck::cast_slice(&Vertex::compute(
                img.dimensions(),
                (size.width, size.height),
                Pan::default(),
                1.0,
            )),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("yume index buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pan: Pan::default(),
            scale: 1.0,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: INDICES.len() as u32,
            texture_bind_group_layout,
            bind_group,
            texture,
        }
    }

    pub fn update_image(&mut self, img: &texture::Rgba8Image) {
        let texture =
            texture::Texture::from_image(&self.device, &self.queue, img, Some("yume texture"));

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("yume diffuse bind group"),
        });

        self.texture = texture;
        self.bind_group = bind_group;
        self.reconfigure_vertex_buffer();
    }

    pub fn reconfigure_vertex_buffer(&mut self) {
        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&Vertex::compute(
                (self.texture.size.width, self.texture.size.height),
                (self.size.width, self.size.height),
                self.pan,
                self.scale,
            )),
        );
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        assert!(size.width != 0 && size.height != 0);

        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.reconfigure_vertex_buffer();
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("yume render encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("yume render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
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
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn surface_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn texture_size(&self) -> PhysicalSize<u32> {
        let size = self.texture.size;
        PhysicalSize {
            width: size.width,
            height: size.height,
        }
    }

    pub fn add_scale(&mut self, d: f32) {
        self.set_scale(self.scale + d);
    }

    pub fn set_scale(&mut self, v: f32) {
        self.scale = v.max(f32::EPSILON);
        self.reconfigure_vertex_buffer();
    }
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    // (width, height)
    pub fn compute(src: (u32, u32), dst: (u32, u32), pan: Pan, scale: f32) -> Vec<Self> {
        let src = (src.0 as f32 * scale, src.1 as f32 * scale);
        let width = dst.0 as f32 / src.0;
        let height = dst.1 as f32 / src.1;
        let mut pan_width = pan.width as f32 / src.0;
        let mut pan_height = pan.height as f32 / src.1;
        if width > 1.0 {
            pan_width -= (dst.0 as f32 - src.0) / (2.0 * src.0);
        }
        if height > 1.0 {
            pan_height -= (dst.1 as f32 - src.1) / (2.0 * src.1);
        }
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [width + pan_width, pan_height],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [pan_width, pan_height],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [pan_width, height + pan_height],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [width + pan_width, height + pan_height],
            },
        ]
    }
}
