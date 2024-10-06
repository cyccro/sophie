use wgpu::vertex_attr_array;

use crate::math::{Rgba, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Rgba,
}
pub trait VertexLayout: Sized {
    fn layout() -> wgpu::VertexBufferLayout<'static>;
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x4
    ];
    pub fn new(position: Vec3, color: Rgba) -> Self {
        Self { position, color }
    }
}
impl VertexLayout for Vertex {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
