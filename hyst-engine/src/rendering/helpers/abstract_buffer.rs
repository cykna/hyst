use crate::core::RenderingCore;
use bytemuck::{Pod,Zeroable};

pub enum BufferType {
    Vertex,
    Uniform,
}

pub struct AbstractBuffer<T> {
    inner: T,
    buffer: wgpu::Buffer,
}

impl<T> AbstractBuffer<T>
where
    T: Pod + Zeroable,
{
    pub fn new(core: &RenderingCore, data: T, buffer_type: BufferType) -> Self {
        Self {
            buffer: match buffer_type {
                BufferType::Vertex => core.create_vertex_buffer(&[data], None),
                BufferType::Uniform => core.create_uniform_buffer(&[data], None),
            },
            inner: data,
        }
    }
    ///Writes the modifications made in the inner value into the buffer
    pub fn write(&mut self, core: &RenderingCore) {
        core.write_buffer_single(&self.inner, &self.buffer);
    }
    ///Writes the given data into this buffer and modifies the inner value, returning the old value
    pub fn write_with(&mut self, core: &RenderingCore, data: T) -> T {
        let out = std::mem::replace(&mut self.inner, data);
        self.write(core);
        out
    }
    pub fn inner_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, const AMOUNT: usize> std::ops::IndexMut<usize> for AbstractBuffer<[T; AMOUNT]>
where
    T: Pod + Zeroable,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T, const AMOUNT: usize> std::ops::Index<usize> for AbstractBuffer<[T; AMOUNT]>
where
    T: Pod + Zeroable,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
} 