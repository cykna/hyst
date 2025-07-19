mod input;
pub use input::*;
use taffy::{Point, Size};
mod shader;
use crate::{background::Background, batch::BatchSubmitter, core::RenderingCore, meshes::Mesh};
use hyst_math::{Rect, vectors::Vec4f32};
pub use shader::*;

#[derive(Debug)]
pub struct Container {
    index: u32,
    instance: ContainerInstance,
}

impl Container {
    pub fn new(bg: Background, rect: Rect, index: u32) -> Self {
        Self {
            index,
            instance: ContainerInstance::new(
                match bg {
                    Background::Transparent => Vec4f32::new(0.0, 0.0, 0.0, 0.0),
                    Background::Solid(c) => c,
                    Background::Gradient { top_left, .. } => top_left,
                },
                rect,
            ),
        }
    }

    ///Retrieves the index of this Container on the instances array of the BatchRenderer that renders it
    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Mesh for Container {
    type Shader = ContainerShader;
    type Vertices = ContainerInput;
    type Instance = ContainerInstance;
}
