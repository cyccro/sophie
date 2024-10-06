#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn get_vec(&self) -> Vec<f32> {
        vec![self.0, self.1, self.2]
    }
}
