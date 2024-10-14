pub struct Transform {
    position: na::Vector3<f32>,
    rotation: na::UnitQuaternion<f32>,
    scale: na::Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: na::Vector3::new(0.0, 0.0, 0.0),
            rotation: na::UnitQuaternion::identity(),
            scale: na::Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn new(
        position: na::Vector3<f32>,
        rotation: na::UnitQuaternion<f32>,
        scale: na::Vector3<f32>,
    ) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
    pub fn position(&self) -> &na::Vector3<f32> {
        &self.position
    }
    pub fn scale(&self) -> &na::Vector3<f32> {
        &self.scale
    }
    pub fn rotation(&self) -> &na::UnitQuaternion<f32> {
        &self.rotation
    }
    pub fn translate(&mut self, translation: &na::Vector3<f32>) {
        self.position += translation;
    }
    pub fn translate_forward(&mut self, scale: Option<f32>) {
        self.position += self.look_at_target() * scale.unwrap_or(1.0);
    }
    pub fn rotate(&mut self, quat: &na::UnitQuaternion<f32>) {
        self.rotation *= quat;
    }
    pub fn scale_uniform_set(&mut self, scale: f32) {
        self.scale.x = scale;
        self.scale.y = scale;
        self.scale.z = scale;
    }
    pub fn scale_set(&mut self, scale: &na::Vector3<f32>) {
        self.scale.copy_from(scale);
    }
    pub fn multiply_scale_uniform(&mut self, scale: f32) {
        self.scale *= scale;
    }
    pub fn multiply_scale(&mut self, scale: &na::Vector3<f32>) {
        self.scale.x *= scale.x;
        self.scale.y *= scale.y;
        self.scale.z *= scale.z;
    }
    pub fn look_at_target(&self) -> na::Vector3<f32> {
        //(0,0,1) = forward
        (self.position + (self.rotation * na::Vector3::new(0.0, 0.0, -1.0))).normalize()
    }
}
