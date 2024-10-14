use std::f32::consts::PI;

use na::{Matrix4, Point3, Vector3};
use nalgebra as na;

use crate::math::OPENGL_TO_WGPU_MATRIX;

use super::CameraInternal;

#[derive(Debug)]
pub struct PerspectiveConfigs {
    pub fov: f32,
    pub far: f32,
    pub near: f32,
    pub aspect: f32,
    pub mat: na::Matrix4<f32>,
}
#[derive(Debug)]
pub struct PerspectiveCamera {
    internal: CameraInternal<PerspectiveConfigs>,
    direction: na::Vector3<f32>,
    target: na::Point3<f32>,
    position: na::Point3<f32>,
    needs_update: bool,
}

impl PerspectiveConfigs {
    pub fn new(mut fov: f32, far: f32, near: f32, aspect: f32) -> Self {
        fov = fov * PI / 180.0;
        let mat = na::Matrix4::new_perspective(aspect, fov, near, far);
        Self {
            fov,
            far,
            near,
            aspect,
            mat,
        }
    }
}

impl PerspectiveCamera {
    pub fn new(
        target: na::Point3<f32>,
        position: na::Point3<f32>,
        configs: PerspectiveConfigs,
    ) -> Self {
        Self {
            internal: CameraInternal {
                projection: na::Matrix4::look_at_rh(
                    &position,
                    &target,
                    &na::Vector3::new(0.0, 1.0, 0.0),
                ),
                config: configs,
            },
            needs_update: false,
            direction: (target - position).normalize(),
            target,
            position,
        }
    }
    pub fn position(&self) -> &na::Point3<f32> {
        &self.position
    }
    pub fn copy_position(&self) -> na::Point3<f32> {
        self.position.clone()
    }
    pub fn translate(&mut self, vec: &Vector3<f32>) {
        self.position += vec;
        self.request_update();
    }
    pub fn projection(&mut self) -> &Matrix4<f32> {
        if self.needs_update {
            self.internal.projection = na::Matrix4::look_at_rh(
                self.position(),
                self.target(),
                &na::Vector3::new(0.0, 1.0, 0.0),
            );
        }
        &self.internal.projection
    }
    pub fn target(&self) -> &Point3<f32> {
        &self.target
    }
    pub fn set_target(&mut self, position: &Point3<f32>) {
        self.target = *position;
        self.direction = (self.position - self.target).normalize();
    }
    pub fn translate_nums(&mut self, x: f32, y: f32, z: f32) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
        self.request_update();
    }
    pub fn direction(&self) -> na::Vector3<f32> {
        self.direction.clone()
    }
    pub fn right(&self) -> na::Vector3<f32> {
        self.direction
            .cross(&na::Vector3::new(0.0, 1.0, 0.0))
            .normalize()
    }
    pub fn translate_forward(&mut self, scale: f32) {
        self.position += self.direction * scale;
        self.request_update();
    }
    pub fn translate_left(&mut self, scale: f32) {
        self.position -= self.right() * scale;
        self.request_update();
    }
    pub fn translate_right(&mut self, scale: f32) {
        self.position += self.right() * scale;
        self.request_update();
    }
    pub fn translate_backward(&mut self, scale: f32) {
        self.position -= self.direction * scale;
        self.request_update();
    }
    pub fn request_update(&mut self) {
        self.needs_update = true;
    }
    pub fn get_view_projection_mat(&mut self) -> na::Matrix4<f32> {
        let perspective = self.internal.config.mat;
        let projection = self.projection();
        return OPENGL_TO_WGPU_MATRIX * (&perspective) * projection;
    }
}
