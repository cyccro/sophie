use wgpu::Queue;

pub trait UniformUpdateable {
    fn update(&mut self, queue: &Queue);
}

pub enum SophieObject {}
