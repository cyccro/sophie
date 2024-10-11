use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Zeroable, PartialEq, Debug, Clone, Copy)]
pub struct Vec2(pub f32, pub f32);

#[repr(C)]
#[derive(Pod, Zeroable, PartialEq, Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);
