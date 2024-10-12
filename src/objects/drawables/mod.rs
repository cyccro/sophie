mod mesh;
pub use mesh::*;

pub trait DrawableUpdateable {
    fn update_data(&mut self, queue: &wgpu::Queue, vp: na::Matrix4<f32>);
}
