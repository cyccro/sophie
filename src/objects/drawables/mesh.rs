use na::{Matrix4, UnitQuaternion, Vector3};
use wgpu::{BindGroup, RenderPass};

use crate::sr_core::{
    helpers::{BindGroupHelper, BufferHelper},
    UniformBuffer, Vertices,
};

use super::DrawableUpdateable;

pub struct Mesh {
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    scale: Vector3<f32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    vpm: UniformBuffer,
    len: u32,
    bindgroup: BindGroup,
}

impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        position: Vector3<f32>,
        rotation: UnitQuaternion<f32>,
        scale: Vector3<f32>,
        vertices: Vertices,
        indices: Vec<u16>,
    ) -> Self {
        let vpm = {
            //(Matrix4::new_translation(&position) * rotation.to_homogeneous())
            let data = Matrix4::<f32>::identity().data.0;
            BufferHelper::uniform_buffer(device, data.as_slice())
        };
        let bindgroup = BindGroupHelper::create_uniform(device, vpm.buffer());
        Self {
            position,
            rotation,
            scale,
            bindgroup,
            vpm,
            vertex_buffer: BufferHelper::vertices_buffer_casted(device, vertices.as_slice()),
            index_buffer: BufferHelper::index_buffer(device, indices.as_slice()),
            len: indices.len() as u32,
        }
    }
    pub fn model_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_translation(&self.position)
            * self.rotation.to_homogeneous()
            * na::Matrix4::new_nonuniform_scaling(&self.scale)
    }
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }
    pub fn resize(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }
    pub fn rotate(&mut self, quaternion: UnitQuaternion<f32>) {
        self.rotation *= quaternion;
    }
    pub fn draw(&self, idx: u32, pass: &mut RenderPass) {
        pass.set_bind_group(idx, &self.bindgroup, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.len, 0, 0..1);
    }
}
impl DrawableUpdateable for Mesh {
    fn update_data(&mut self, queue: &wgpu::Queue, vp: na::Matrix4<f32>) {
        let data = vp * self.model_matrix();
        self.vpm.update(queue, &data.data.0);
    }
}
