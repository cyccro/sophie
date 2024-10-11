use wgpu::{Device, Queue};

use crate::sr_core::helpers::{BindGroupInfo, BindGroupKind, HasBindgroup};

#[derive(Debug)]
pub struct UniformBuffer {
    buffer: wgpu::Buffer,
    size: usize,
    pub info: BindGroupInfo,
}
impl UniformBuffer {
    pub fn new(device: &Device, buffer: wgpu::Buffer) -> Self {
        Self {
            info: BindGroupInfo::new(device, &BindGroupKind::UNIFORM(&buffer)),
            size: buffer.size() as usize,
            buffer,
        }
    }
    pub fn update(&self, queue: &Queue, data: &[u8]) {
        queue.write_buffer(&self.buffer, 0, data);
    }
}
impl HasBindgroup for UniformBuffer {
    fn info(&self, device: &Device) -> BindGroupInfo {
        BindGroupInfo::new(device, &BindGroupKind::UNIFORM(&self.buffer))
    }
}
