use bytemuck::Pod;
use wgpu::Queue;

#[derive(Debug)]
pub struct UniformBuffer {
    buffer: wgpu::Buffer,
}
impl UniformBuffer {
    pub fn new(buffer: wgpu::Buffer) -> Self {
        Self { buffer }
    }
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
    pub fn update_casted(&self, queue: &Queue, data: &[u8]) {
        queue.write_buffer(&self.buffer, 0, data);
    }
    pub fn update<T: Pod>(&self, queue: &Queue, data: &[T]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }
}
