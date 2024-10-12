#[repr(C)]
#[derive(Pod, Zeroable, Debug, Clone, Copy)]
pub struct CameraData {
    data: [[f32; 4]; 4],
}

#[derive(Debug)]
pub struct CameraInternal<T> {
    projection: na::Matrix4<f32>,
    config: T,
}
impl CameraData {
    pub fn update(&mut self, data: [[f32; 4]; 4]) {
        self.data = data;
    }
}
mod ortho;
mod perspective;
use bytemuck::{Pod, Zeroable};
pub use perspective::*;
