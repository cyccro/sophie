mod transform;
use bytemuck::{Pod, Zeroable};
pub use transform::Transform;

//structs
#[repr(C)]
#[derive(Pod, Zeroable, PartialEq, Debug, Clone, Copy)]
pub struct Vec2(pub f32, pub f32);

#[repr(C)]
#[derive(Pod, Zeroable, PartialEq, Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

//constants
pub const OPENGL_TO_WGPU_MATRIX: na::Matrix4<f32> = na::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);
