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
    pub fn index_buffer_casted(device: &wgpu::Device, data: &[u8]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::INDEX,
        })
    }
    pub fn vertices_buffer_casted(device: &wgpu::Device, data: &[u8]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
    pub fn index_buffer<T: Pod>(device: &wgpu::Device, data: &[T]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
    pub fn vertices_buffer<T: Pod>(device: &wgpu::Device, data: &[T]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
    pub fn uniform<T: Pod>(device: &wgpu::Device, data: &[T]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        })
    }
    pub fn uniform_buffer<T: Pod>(device: &wgpu::Device, data: &[T]) -> UniformBuffer {
        UniformBuffer::new(Self::uniform(device, data))
    }
}
