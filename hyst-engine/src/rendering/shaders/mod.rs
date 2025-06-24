use std::sync::Arc;

use bitmask::bitmask;
pub mod events;
use wgpu::{BindGroup, BindGroupLayout, RenderPipeline, ShaderModule};

use crate::rendering::basics::BindGroupAndLayoutConfig;

bitmask! {
    pub mask ShaderRenderMethod:u8 where flags ShaderRenderingStyle {

        //Type
        PointList = 0,
        LineList = 1,
        LineStrip = 2,
        TriangleList = 3,
        TriangleStrip = 4,

        //Cw = 0 << 3,
        Ccw = 1 << 3,

        //Front = 0 << 4,
        Back = 1 << 4,

        //Fill = 0 << 5,
        Line = 1 << 5,
        Point = 2 << 5
        /*
        PPFDTT
        T = Type
        D = Direction
        F = Face
        P = Polygon Mode
        */
    }
}
impl ShaderRenderMethod {
    pub const TriangleCcwBack: Self = Self {
        mask: (ShaderRenderingStyle::Ccw as u8
            | ShaderRenderingStyle::TriangleList as u8
            | ShaderRenderingStyle::Back as u8),
    };
    pub fn get_primitive_state(&self) -> wgpu::PrimitiveState {
        let this = self.mask as u8;
        wgpu::PrimitiveState {
            topology: match this & 0b11 {
                0b00 => wgpu::PrimitiveTopology::PointList,
                0b01 => wgpu::PrimitiveTopology::LineList,
                0b10 => wgpu::PrimitiveTopology::LineStrip,
                0b11 => wgpu::PrimitiveTopology::TriangleList,
                _ => wgpu::PrimitiveTopology::TriangleStrip,
            },
            strip_index_format: None,
            front_face: match this & 0b100 {
                0 => wgpu::FrontFace::Cw,
                _ => wgpu::FrontFace::Ccw,
            },
            cull_mode: Some(match this & 0b1000 {
                0 => wgpu::Face::Front,
                _ => wgpu::Face::Back,
            }),
            polygon_mode: match (this & 0b110000) >> 5 {
                0 => wgpu::PolygonMode::Fill,
                1 => wgpu::PolygonMode::Line,
                _ => wgpu::PolygonMode::Point,
            },
            unclipped_depth: false,
            conservative: false,
        }
    }
}

pub struct ShaderCreationOptions<'a> {
    pub source: &'a str,
    pub bind_group_configs: Vec<Vec<BindGroupAndLayoutConfig<'a>>>,
    pub rendering_style: ShaderRenderMethod,
    pub name: String,
}

pub trait ShaderInput {
    const LAYOUT: wgpu::VertexBufferLayout<'static>;
}

pub trait HystShader {
    fn module(&self) -> &Arc<wgpu::ShaderModule>;
    fn pipeline(&self) -> &wgpu::RenderPipeline;
    fn bind_group_layouts(&self) -> Option<&[wgpu::BindGroupLayout]> {
        None
    }
    fn bind_groups(&self) -> &[wgpu::BindGroup] {
        &[]
    }
}
pub trait HystConstructor: HystShader {
    fn new(
        module: Arc<ShaderModule>,
        bindgroups: Vec<BindGroup>,
        layouts: Vec<BindGroupLayout>,
        pipeline: Arc<RenderPipeline>,
    ) -> Self
    where
        Self: Sized;

    fn shader_inputs() -> Vec<wgpu::VertexBufferLayout<'static>>;

    fn name() -> &'static str;
}
