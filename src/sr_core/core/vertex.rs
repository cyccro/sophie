use bytemuck::AnyBitPattern;
use wgpu::vertex_attr_array;

use crate::math::{Rgba, Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Rgba,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturedVertex {
    pub position: Vec3,
    pub uv: Vec2,
}
pub enum Vertices {
    Vertex(Vec<Vertex>),
    Textured(Vec<TexturedVertex>),
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
impl TexturedVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2
    ];
    pub fn new(position: Vec3, uv: Vec2) -> Self {
        Self { position, uv }
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
impl VertexLayout for TexturedVertex {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
impl Vertices {
    // size of the struct it contains in bytes
    pub fn memsize(&self) -> usize {
        match self {
            Self::Vertex(_) => std::mem::size_of::<Vertex>(),
            Self::Textured(_) => std::mem::size_of::<TexturedVertex>(),
        }
    }
    // total memory size it this struct contains
    pub fn size(&self) -> usize {
        self.len() * self.memsize()
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Vertex(v) => v.len(),
            Self::Textured(v) => v.len(),
        }
    }
    pub fn layout(&self) -> wgpu::VertexBufferLayout {
        match self {
            Vertices::Textured(_) => TexturedVertex::layout(),
            Vertices::Vertex(_) => Vertex::layout(),
        }
    }
    pub fn as_slice<T: AnyBitPattern>(&self) -> &[T] {
        match self {
            Self::Vertex(v) => bytemuck::cast_slice(v.as_slice()),
            Self::Textured(v) => bytemuck::cast_slice(v.as_slice()),
        }
    }
}
