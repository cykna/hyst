use std::{collections::HashMap, sync::Arc};

use crate::rendering::basics::*;
use bytemuck::{Pod, Zeroable};
use glyphon::{Cache, TextAtlas, TextRenderer};
use hyst_math::vectors::Rgba;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{
    Adapter, BackendOptions, Backends, BindGroup, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutEntry, Buffer, BufferDescriptor, BufferUsages, Device, Instance, InstanceFlags,
    Origin3d, Queue, RenderPipeline, ShaderModule, ShaderStages, Surface, SurfaceConfiguration,
    TextureDescriptor, TextureViewDescriptor, VertexBufferLayout,
};
use winit::window::Window;

use crate::shaders::{HystConstructor, ShaderCreationOptions};

use super::elements::HystElement;

pub struct RenderingCore {
    instance: Instance,
    surface: Surface<'static>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    text_renderer: TextRenderer,
    pipelines: HashMap<&'static str, Arc<RenderPipeline>>,
    shaders: HashMap<&'static str, Arc<ShaderModule>>,
}

impl RenderingCore {
    pub fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: Backends::all(),
            flags: InstanceFlags::debugging(),
            backend_options: BackendOptions::default(),
        });
        let surface = unsafe {
            instance
                .create_surface_unsafe({
                    wgpu::SurfaceTargetUnsafe::RawHandle {
                        raw_display_handle: window.display_handle().unwrap().as_raw(),
                        raw_window_handle: window.window_handle().unwrap().as_raw(),
                    }
                })
                .unwrap()
        };
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::POLYGON_MODE_LINE,
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
        }))
        .unwrap();
        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);
        let mut texture_atlas =
            TextAtlas::new(&device, &queue, &Cache::new(&device), config.format);
        Self {
            instance,
            surface,
            adapter,
            queue,
            config,
            text_renderer: TextRenderer::new(
                &mut texture_atlas,
                &device,
                wgpu::MultisampleState::default(),
                None,
            ),
            device,
            pipelines: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn write_buffer<T>(&self, data: &[T], buffer: &Buffer)
    where
        T: Pod + Zeroable,
    {
        self.queue
            .write_buffer(buffer, 0, bytemuck::cast_slice(data));
    }
    pub fn write_buffer_single<T>(&self, data: &T, buffer: &Buffer)
    where
        T: Pod + Zeroable,
    {
        self.queue.write_buffer(buffer, 0, bytemuck::bytes_of(data));
    }

    pub fn create_shader<S>(&mut self, options: ShaderCreationOptions) -> S
    where
        S: HystConstructor + Sized,
    {
        let (bind_groups, layouts) =
            self.create_bind_groups_and_layouts(options.bind_group_configs, Some(&options.name));
        if let Some(module) = self.shaders.get(S::name()) {
            let pipeline = self.pipelines.get(S::name()).unwrap();
            S::new(module.clone(), bind_groups, layouts, pipeline.clone())
        } else {
            let module = Arc::new(self.create_module(&options.source, Some(&options.name)));
            let pipeline = self.create_default_pipeline(
                &module,
                Some(&options.name),
                options.rendering_style.get_primitive_state(),
                &layouts.iter().map(|l| l).collect::<Vec<&_>>(),
                S::shader_inputs(),
            );
            let pipeline = Arc::new(pipeline);
            self.pipelines.insert(S::name(), pipeline.clone());
            self.shaders.insert(S::name(), module.clone());
            S::new(module, bind_groups, layouts, pipeline)
        }
    }

    ///Gets the size of the surface
    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    ///Used to create a default pipeline for the given shader module. It has default configs, but may not be used for every shader
    /// Default configs are:
    /// Entry points: vertex_main and fragment_main
    /// Shader input is defined by the given 'inputs' parameter
    /// The way is going to draw is defined by the given 'draw_method'
    pub fn create_default_pipeline(
        &self,
        module: &wgpu::ShaderModule,
        label: Option<&str>,
        draw_method: wgpu::PrimitiveState,
        layouts: &[&BindGroupLayout],
        inputs: Vec<VertexBufferLayout<'_>>,
    ) -> wgpu::RenderPipeline {
        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&self.device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label,
                        bind_group_layouts: &layouts,
                        push_constant_ranges: &[],
                    },
                )),
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some("vertex_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &inputs,
                },
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some("fragment_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    })],
                }),
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                cache: None,
                multiview: None,
                primitive: draw_method,
            })
    }
    pub fn create_module(&self, source: &str, label: Option<&str>) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
    }

    ///Creates bindgroups and bind group layouts as required. If no bindgroups are required, assert!(out.0.is_empty())
    /// Each vector is understood as a bind group, so 'vec![vec![config,config2,config3], vec![config4,config5,config6]]'
    /// will be understood as 2 bindgroups, 1 with the 3 first configs, and the second with the others. When using shader.draw() these will be understood as
    /// 0 and 1, so their index is their number order on the wgsl
    pub fn create_bind_groups_and_layouts<'a>(
        &self,
        configs: Vec<Vec<BindGroupAndLayoutConfig<'a>>>,
        label: Option<&str>,
    ) -> (Vec<BindGroup>, Vec<BindGroupLayout>) {
        configs
            .iter()
            .enumerate()
            .map(|(idx, configs)| {
                self.create_bindgroup_and_layout(
                    label.map(|v| format!("{v} at idx {idx}")).as_deref(),
                    configs.as_slice(),
                )
            })
            .unzip()
    }

    pub fn create_bindgroup_and_layout(
        &self,
        label: Option<&str>,
        configs: &[BindGroupAndLayoutConfig],
    ) -> (BindGroup, BindGroupLayout) {
        let layout = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label,
                entries: &configs
                    .iter()
                    .enumerate()
                    .map(|(binding, config)| match config {
                        BindGroupAndLayoutConfig::Uniform(visibility, _) => BindGroupLayoutEntry {
                            binding: binding as u32,
                            visibility: *visibility,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        BindGroupAndLayoutConfig::Texutre(dimension, sample_type, _) => {
                            BindGroupLayoutEntry {
                                binding: binding as u32,
                                visibility: ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    sample_type: *sample_type,
                                    view_dimension: *dimension,
                                    multisampled: false,
                                },
                                count: None,
                            }
                        }
                        BindGroupAndLayoutConfig::Sampler(config, _) => BindGroupLayoutEntry {
                            binding: binding as u32,
                            visibility: ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(*config),
                            count: None,
                        },
                    })
                    .collect::<Vec<BindGroupLayoutEntry>>(),
            });
        let bindgroup = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &layout,
            entries: &configs
                .iter()
                .enumerate()
                .map(|(binding, config)| match config {
                    BindGroupAndLayoutConfig::Uniform(_, buffer) => BindGroupEntry {
                        binding: binding as u32,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                    BindGroupAndLayoutConfig::Texutre(_, _, txt) => BindGroupEntry {
                        binding: binding as u32,
                        resource: wgpu::BindingResource::TextureView(txt),
                    },
                    BindGroupAndLayoutConfig::Sampler(_, sampler) => BindGroupEntry {
                        binding: binding as u32,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                })
                .collect::<Vec<BindGroupEntry>>(),
        });
        (bindgroup, layout)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn create_vertex_buffer<T>(&self, data: &[T], label: Option<&str>) -> Buffer
    where
        T: Pod + Zeroable,
    {
        let data = bytemuck::cast_slice(data);
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: data.len() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&buffer, 0, data);
        buffer
    }

    pub fn create_index_buffer(&self, data: &[u16], label: Option<&str>) -> Buffer {
        let byte_data = bytemuck::cast_slice(&data);
        let buffer = self.device.create_buffer(&BufferDescriptor {
            label,
            size: byte_data.len() as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&buffer, 0, byte_data);
        buffer
    }

    pub fn create_uniform_buffer<T>(&self, data: &[T], label: Option<&str>) -> Buffer
    where
        T: Pod + Zeroable,
    {
        let data = bytemuck::cast_slice(data);
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: data.len() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&buffer, 0, data);
        buffer
    }

    pub fn create_image(&self, size: (u32, u32), data: &[u8]) -> GpuImage {
        let size3d = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&TextureDescriptor {
            label: None,
            size: size3d,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.0),
                rows_per_image: Some(size.1),
            },
            size3d,
        );
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        GpuImage::new(texture, sampler, view)
    }

    pub fn draw(&self, elements: &[&Box<dyn HystElement>], bg: Rgba) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear({
                            wgpu::Color {
                                r: bg.x() as f64,
                                g: bg.y() as f64,
                                b: bg.z() as f64,
                                a: bg.w() as f64,
                            }
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            for element in elements {
                element.render(&mut render_pass);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
