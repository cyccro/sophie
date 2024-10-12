use std::ops::Index;

use wgpu::Queue;

use crate::objects::{
    camera::PerspectiveCamera,
    drawables::{DrawableUpdateable, Mesh},
    SophieKeyboardControllable,
};

pub struct Entities {
    meshes: Vec<Mesh>,
}

impl Entities {
    pub fn with_meshes(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }
    pub fn new() -> Self {
        Self {
            meshes: Vec::with_capacity(1028),
        }
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }
    pub fn get_mesh(&self, idx: usize) -> Option<&Mesh> {
        self.meshes.get(idx)
    }
    pub fn get_mut_mesh(&mut self, idx: usize) -> Option<&mut Mesh> {
        self.meshes.get_mut(idx)
    }
    pub fn update(&mut self, queue: &Queue, camera: &mut PerspectiveCamera) {
        for mesh in self.meshes.iter_mut() {
            mesh.update_data(queue, camera.get_view_projection_mat());
        }
    }
}
