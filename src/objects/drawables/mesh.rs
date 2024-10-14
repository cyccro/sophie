use na::UnitQuaternion;
use wgpu::RenderPass;

use crate::{
    math::Transform,
    objects::{
        materials::{Material, MaterialUpdateable},
        ShaderInfo,
    },
    sr_core::{helpers::BufferHelper, UniformBuffer, Vertices},
};

use super::DrawableUpdateable;

pub struct Mesh {
    vpm: UniformBuffer,
    material: Material,
    transform: Transform,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    len: u32,
}

impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        transform: Transform,
        vertices: Vertices,
        indices: Vec<u16>,
        info: ShaderInfo,
        program_id: &str,
    ) -> Self {
        let vpm = BufferHelper::uniform_buffer(device, &na::Matrix4::<f32>::identity().data.0);
        Self {
            material: Material::from_program(program_id.to_string(), device, info, vpm.buffer()),
            vpm,
            transform,
            vertex_buffer: BufferHelper::vertices_buffer_casted(device, vertices.as_slice()),
            index_buffer: BufferHelper::index_buffer(device, indices.as_slice()),
            len: indices.len() as u32,
        }
    }
    pub fn transform(&self) -> &Transform {
        &self.transform
    }
    pub fn mut_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }
    pub fn rotate(&mut self, quat: &UnitQuaternion<f32>) {
        self.transform.rotate(quat);
    }
    pub fn translate(&mut self, direction: &na::Vector3<f32>) {
        self.transform.translate(direction);
    }
    pub fn position(&self) -> &na::Vector3<f32> {
        self.transform.position()
    }
    pub fn scale(&self) -> &na::Vector3<f32> {
        self.transform.scale()
    }
    pub fn rotation(&self) -> &na::UnitQuaternion<f32> {
        self.transform.rotation()
    }
    pub fn material_id(&self) -> &str {
        self.material.id()
    }
    pub fn model_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_translation(&self.position())
            * self.rotation().to_homogeneous()
            * na::Matrix4::new_nonuniform_scaling(&self.scale())
    }
    pub fn draw(&self, pass: &mut RenderPass) {
        for (idx, (group, _)) in self.material.bindgroups().iter().enumerate() {
            pass.set_bind_group(idx as u32, group, &[]);
        }
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.len, 0, 0..1);
    }
}
impl DrawableUpdateable for Mesh {
    fn update_data(&mut self, queue: &wgpu::Queue, vp: na::Matrix4<f32>) {
        self.vpm.update(queue, &(vp * self.model_matrix()).data.0);
        self.material.update(queue);
    }
}
