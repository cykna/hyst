use super::{Container, ContainerShader, InstanceData};
use crate::{
    core::RenderingCore,
    shaders::{ShaderCreationOptions, ShaderRenderMethod},
    AbstractBuffer, BindGroupAndLayoutConfig, BufferType,
};
use std::collections::HashMap;
use wgpu::{Buffer, RenderPass};

// Gerencia batching de containers
pub struct BatchRenderer {
    shader: ContainerShader,
    vertex_buffer: Buffer,
    instance_buffers: HashMap<i32, Buffer>,
    instance_counts: HashMap<i32, u32>,
}

impl BatchRenderer {
    pub fn new(core: &mut RenderingCore, containers: &[Container]) -> Self {
        // Cria buffer de vértices com todos os containers
        let vertex_data: Vec<_> = containers.iter().flat_map(|c| *c.vertices.inner()).collect();
        let vertex_buffer = core.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Container Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // Cria shader
        let shader = core.create_shader(ShaderCreationOptions {
            source: &std::fs::read_to_string("./shaders/container.wgsl").unwrap(),
            bind_group_configs: containers
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    vec![
                        BindGroupAndLayoutConfig::Uniform(
                            wgpu::ShaderStages::VERTEX,
                            c.screen_size.inner_buffer(),
                        ),
                        BindGroupAndLayoutConfig::Uniform(
                            wgpu::ShaderStages::VERTEX,
                            c.rect_buffer().inner_buffer(),
                        ),
                    ]
                })
                .collect(),
            rendering_style: ShaderRenderMethod::TriangleStrip,
            name: "container".to_string(),
        });

        // Agrupa por profundidade
        let mut instance_buffers = HashMap::new();
        let mut instance_counts = HashMap::new();
        let mut depth_map: HashMap<i32, Vec<u32>> = HashMap::new();
        for (i, container) in containers.iter().enumerate() {
            depth_map.entry(container.depth()).or_insert(Vec::new()).push(i as u32);
        }

        // Cria buffers de instâncias por profundidade
        for (depth, indices) in depth_map {
            let instance_data: Vec<InstanceData> = indices.iter().map(|&i| InstanceData { index: i }).collect();
            let buffer = core.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Container Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            });
            instance_buffers.insert(depth, buffer);
            instance_counts.insert(depth, indices.len() as u32);
        }

        Self {
            shader,
            vertex_buffer,
            instance_buffers,
            instance_counts,
        }
    }

    pub fn render<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(self.shader.pipeline());

        // Configura buffer de vértices
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // Ordena profundidades em ordem decrescente
        let mut depths: Vec<i32> = self.instance_buffers.keys().cloned().collect();
        depths.sort_by(|a, b| b.cmp(a));

        // Desenha cada profundidade
        for depth in depths {
            if let (Some(buffer), Some(count)) = (
                self.instance_buffers.get(&depth),
                self.instance_counts.get(&depth),
            ) {
                pass.set_vertex_buffer(1, buffer.slice(..));
                for (i, bind_group) in self.shader.bind_groups().iter().enumerate() {
                    pass.set_bind_group(i as u32, bind_group, &[]);
                }
                pass.draw(0..4, 0..*count);
            }
        }
    }
}
