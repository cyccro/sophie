mod material;
pub use material::*;

pub trait MaterialUpdateable {
    fn update(&mut self, queue: &wgpu::Queue);
}
