use bytemuck::Pod;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages,
};

use crate::sr_core::UniformBuffer;

pub struct BufferHelper;

impl BufferHelper {
    pub fn create_buffer<T: Pod>(
        device: &wgpu::Device,
        data: &[T],
        usage: BufferUsages,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage,
        })
    }
    pub fn uniform_buffer<T: Pod>(device: &wgpu::Device, data: &[T]) -> UniformBuffer {
        UniformBuffer::new(
            device,
            Self::create_buffer(device, data, BufferUsages::UNIFORM | BufferUsages::COPY_DST),
        )
    }
}
