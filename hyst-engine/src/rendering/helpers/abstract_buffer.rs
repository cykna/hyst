use crate::core::RenderingCore;
use bytemuck::{Pod, Zeroable};

pub enum BufferType {
    Vertex,
    Uniform,
}
#[derive(Debug)]
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
        core.write_buffer_single(&self.inner, &self.buffer, None);
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

pub struct AbstractVecBuffer<T> {
    inner: Vec<T>,
    buffer: wgpu::Buffer,
}

impl<T> AbstractVecBuffer<T>
where
    T: Pod + Zeroable,
{
    ///Creates a new empty instanced Abstract Vec Buffer with capacity to allocate `size` instances.
    ///As the normal is for buffers containing data on init, thus, having a size,
    ///So this is understood by this engine to be a buffer that might be used for instanced drawing.
    pub fn empty(core: &RenderingCore, size: u64) -> Self {
        Self {
            buffer: core.create_instance_buffer::<T>(Some(size), None),
            inner: Vec::with_capacity(size as usize),
        }
    }

    ///Creates a new AbstractBuffer containing the given Vec as inner data.
    pub fn new_vec(core: &RenderingCore, data: Vec<T>, buffer_type: BufferType) -> Self {
        Self {
            buffer: match buffer_type {
                BufferType::Vertex => core.create_vertex_buffer(&data, None),
                BufferType::Uniform => core.create_uniform_buffer(&data, None),
            },
            inner: data,
        }
    }

    ///Write the given data on the given index, both on the inner vector and the buffer.
    ///The indexing on the buffer works the same way it would work on a conventional vector, so indexing by 3 will start writing from `3 * sizeof(T)` on the buffer
    pub fn write_single(&mut self, core: &RenderingCore, index: u64, data: T) {
        core.write_buffer_single(&data, &self.buffer, Some(index));
        self.inner[index as usize] = data;
    }

    ///Writes the modifications made in the inner value into the buffer
    pub fn write_vec(&self, core: &RenderingCore) {
        core.write_buffer(self.inner.as_slice(), &self.buffer);
    }
    ///Writes the given data into this buffer and modifies the inner value, returning the old value
    pub fn write_with_vec(&mut self, core: &RenderingCore, data: Vec<T>) -> Vec<T> {
        let out = std::mem::replace(&mut self.inner, data);
        self.write_vec(core);
        out
    }
    pub fn inner_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
    pub fn inner(&self) -> &Vec<T> {
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

impl<T> std::ops::IndexMut<usize> for AbstractVecBuffer<T>
where
    T: Pod + Zeroable,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T> std::ops::Index<usize> for AbstractVecBuffer<T>
where
    T: Pod + Zeroable,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
