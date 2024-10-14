use crate::{
    math::Rgba,
    objects::ShaderInfo,
    sr_core::{helpers::BufferHelper, UniformBuffer},
};
use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, BindGroupLayout, Device};

use super::MaterialUpdateable;

#[derive(Debug)]
pub struct Material {
    id: String,
    buffer: UniformBuffer,
    bindgroups: Vec<(BindGroup, BindGroupLayout)>,
    data: MaterialData,
}
#[repr(C)]
#[derive(Debug, Pod, Zeroable, Clone, Copy)]
pub struct MaterialData {
    color: Rgba,
    metallic: f32,
    roughness: f32,
    padding: [f32; 2],
}
impl Material {
    //creates into the materials list
    pub fn from_program(
        id: String,
        device: &Device,
        infos: ShaderInfo,
        vpmbuffer: &wgpu::Buffer,
    ) -> Self {
        let data = MaterialData {
            color: Rgba(1.0, 1.0, 1.0, 1.0),
            metallic: 1.0,
            roughness: 1.0,
            padding: [0.0, 0.0],
        };
        let buffer = BufferHelper::uniform_buffer(device, &[data]);
        Self {
            id,
            bindgroups: infos.create_bindgroups(device, vpmbuffer, buffer.buffer()),
            buffer,
            data,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn bindgroups(&self) -> &Vec<(BindGroup, BindGroupLayout)> {
        &self.bindgroups
    }
}
impl MaterialUpdateable for Material {
    fn update(&mut self, queue: &wgpu::Queue) {
        self.buffer.update(queue, &[self.data]);
    }
}
