#[derive(Debug)]
pub struct CameraInternal<T> {
    projection: na::Matrix4<f32>,
    config: T,
}
mod ortho;
mod perspective;
pub use perspective::*;
