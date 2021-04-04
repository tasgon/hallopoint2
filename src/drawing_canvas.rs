use std::sync::Arc;

use acidalia::{Element, Engine, Nametag, ShaderKind, graphics::AsExtent, shaders::RenderTags, wgpu::{self, BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, RenderPipeline, RenderPipelineDescriptor, Texture, TextureDescriptor, TextureFormat, TextureUsage, TextureView, TextureViewDescriptor}, winit::event::{Event, WindowEvent}};

#[derive(Nametag)]
pub enum DrawShaders {
    RenderVert,
    RenderFrag,
    DrawVert,
    DrawFrag,
}

pub fn init_draw_shaders(e: &mut Engine) {
    let ss = &mut e.shader_state;
    ss.load_file(
        DrawShaders::RenderVert,
        "./src/gl/canvas_render.vert",
        "main",
        ShaderKind::Vertex,
        None,
    );
    ss.load_file(
        DrawShaders::RenderVert,
        "./src/gl/canvas_render.frag",
        "main",
        ShaderKind::Fragment,
        None,
    );
}

pub struct DrawingCanvas {
    tex: Vec<(Texture, TextureView, BindGroup)>,
    pipeline: Arc<RenderPipeline>,
}

impl DrawingCanvas {
    pub fn new(e: &mut Engine) -> Self {
        let gs = &mut e.graphics_state;
        let sampler = gs.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group_layout =
            gs.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                comparison: false,
                                filtering: true,
                            },
                            count: None,
                        },
                    ],
                    label: Some("iced bgl"),
                });
        let tex = (1..2)
            .map(|i| {
                let t = gs.device.create_texture(&TextureDescriptor {
                    label: Some(&format!("canvas tex {}", i)),
                    size: gs.get_size().as_extent(1),
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: acidalia::wgpu::TextureDimension::D2,
                    format: TextureFormat::Rgba8UnormSrgb,
                    usage: TextureUsage::SAMPLED | TextureUsage::RENDER_ATTACHMENT,
                });
                let v = t.create_view(&Default::default());
                let bg = gs.device.create_bind_group(&BindGroupDescriptor {
                    label: Some(&format!("canvas bg {}", i)),
                    layout: &bind_group_layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::TextureView(&v),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Sampler(&sampler),
                        }
                    ],

                });
                (t, v, bg)
            })
            .collect::<Vec<_>>();
        let pipeline_layout = gs
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("canvas layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let format = gs.swapchain_descriptor.format;
        let pipeline = e.shader_state.pipeline(gs, move |dev, shaders|
            dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("canvas pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shaders.vertex,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shaders.fragment,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format,
                        alpha_blend: wgpu::BlendState::REPLACE,
                        color_blend: wgpu::BlendState::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    polygon_mode: wgpu::PolygonMode::Fill,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
        }), RenderTags::new(DrawShaders::RenderVert, DrawShaders::RenderFrag));
        Self {
            tex,
            pipeline,
        }
    }
}

impl Element for DrawingCanvas {
    type Data = crate::Data;

    fn update(
        &mut self,
        engine: &mut acidalia::Engine,
        data: &mut Self::Data,
        event: &acidalia::winit::event::Event<()>,
    ) {
        if let Event::WindowEvent {
            event: ev,
            ..
        } = &event {
            if let WindowEvent::Touch(tev) = ev {
                self.tex.swap(0, 1);
            }
        }
    }

    fn render<'a: 'rp, 'rp>(
        &'a mut self,
        engine: &mut acidalia::Engine,
        data: &mut Self::Data,
        frame: &acidalia::wgpu::SwapChainFrame,
        render_pass: &mut acidalia::wgpu::RenderPass<'rp>,
    ) {
        render_pass.set_pipeline(&self.pipeline);
    }
}
