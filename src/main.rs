extern crate nalgebra as na;

use std::process::ExitCode;

use na::Point3;
use sophie::Sophie;
use sr_core::{
    helpers::{BindGroupInfo, HasBindgroup},
    SophieBufferDataDescriptor, TexturedVertex, Vertices,
};
use tests::TestHandler1;
use tracing_subscriber;
mod errors;
mod math;
mod objects;
mod sophie;
mod sr_core;
mod tests;

#[tokio::main]
async fn main() -> ExitCode {
    #[cfg(debug_assertions)]
    //tracing_subscriber::fmt::init();
    let sdl = sdl2::init().unwrap();
    let mut sophie = Sophie::new(&sdl, "Window", (1080, 720)).await.unwrap();
    let shader = sophie
        .wgpu
        .create_shader_from_file(std::path::Path::new("./src/shaders/test4.wgsl"))
        .unwrap();
    let img = sophie
        .wgpu
        .create_texture_from_file(std::path::Path::new("./images/anime.jpeg"))
        .unwrap();
    let vertices = Vertices::Textured(vec![
        TexturedVertex::new(math::Vec3(-0.5, 0.5, 0.0), math::Vec2(0.0, 0.0)),
        TexturedVertex::new(math::Vec3(0.5, 0.5, 0.0), math::Vec2(1.0, 0.0)),
        TexturedVertex::new(math::Vec3(-0.5, -0.5, 0.0), math::Vec2(0.0, 1.0)),
        TexturedVertex::new(math::Vec3(0.5, -0.5, 0.0), math::Vec2(1.0, 1.0)),
    ]);
    let camera = sophie.wgpu.create_perspective_camera(
        Point3::new(0.0, 1.0, 0.0),
        Point3::origin(),
        120.0,
        1000.0,
        0.1,
    );
    let info = vec![img.info(sophie.device()), camera.info(sophie.device())];
    sophie.wgpu.add_program_from(
        &shader,
        Some(SophieBufferDataDescriptor {
            attribute_deffinitions: None,
            vertices,
            info,
            indices: Some(vec![2, 1, 0, 3, 1, 2]),
        }),
    );
    sophie.listen(
        &sdl,
        &mut TestHandler1 {
            camera,
            objects: vec![],
        },
    )
}

#[cfg(test)]
mod testing {
    enum TestErrs {
        Idk,
    }
    struct TestHandler;
    impl SophieHandler<TestErrs> for TestHandler {
        fn update(
            &self,
            _dt: std::time::Duration,
            sophie: &mut Sophie<TestErrs>,
        ) -> crate::sophie::SophieEventResult<TestErrs> {
            if let Err(_) = sophie.wgpu.render() {
                SophieEventResult::Error(TestErrs::Idk)
            } else {
                SophieEventResult::Exit
            }
        }
        fn resize(
            &self,
            _sophie: &mut Sophie<TestErrs>,
            _x: i32,
            _y: i32,
        ) -> SophieEventResult<TestErrs> {
            SophieEventResult::Error(TestErrs::Idk)
        }
        fn before_exit(
            &self,
            _sophie: &mut Sophie<TestErrs>,
            default_request: bool,
            cancel: &mut bool,
        ) {
            *cancel = !default_request;
        }
        fn fallback_error(&self, err: TestErrs, _sophie: &mut Sophie<TestErrs>) {
            match err {
                TestErrs::Idk => panic!("Got some error"),
            }
        }
    }
    use crate::{
        math,
        sophie::{SophieEventResult, SophieHandler},
        sr_core::{SophieBufferDataDescriptor, TexturedVertex, Vertex, Vertices},
        Sophie,
    };
    use sdl2::Sdl;
    async fn test_init() -> Result<(Sophie<'static, TestErrs>, Sdl), ()> {
        let sdl = sdl2::init().map_err(|_| ())?;
        Ok((
            Sophie::new(&sdl, "Window From Sophie", &math::Vec2(1080.0, 720.0)).await,
            sdl,
        ))
    }
    #[tokio::test]
    async fn draw_square() -> Result<(), ()> {
        let (mut sophie, sdl) = test_init().await.unwrap();
        let vertices = Vertices::Vertex(vec![
            Vertex::new(math::Vec3(-0.5, 0.5, 0.0), math::Rgba(0.0, 1.0, 0.0, 1.0)),
            Vertex::new(math::Vec3(0.5, 0.5, 0.0), math::Rgba(1.0, 0.0, 0.0, 1.0)),
            Vertex::new(math::Vec3(-0.5, -0.5, 0.0), math::Rgba(0.0, 0.0, 1.0, 1.0)),
            Vertex::new(math::Vec3(0.5, -0.5, 0.0), math::Rgba(1.0, 1.0, 1.0, 1.0)),
        ]);
        let shader = sophie
            .wgpu
            .create_shader_from_file(std::path::Path::new("./src/shaders/test2.wgsl"))
            .unwrap();
        sophie.wgpu.add_program_from(
            &shader,
            Some(SophieBufferDataDescriptor {
                layouts: None,
                vertices,
                indices: Some(vec![2, 1, 0, 3, 1, 2]),
                texture: None,
            }),
        );
        assert_eq!(std::mem::size_of::<[Vertex; 4]>(), (12 + 16) * 4);
        sophie.listen(&sdl, &TestHandler);
        Ok(())
    }
    #[tokio::test]
    async fn draw_textured_test() -> Result<(), ()> {
        let (mut sophie, sdl) = test_init().await.unwrap();
        let vertices = Vertices::Textured(vec![
            TexturedVertex::new(math::Vec3(-0.5, 0.5, 0.0), math::Vec2(0.0, 0.0)),
            TexturedVertex::new(math::Vec3(0.5, 0.5, 0.0), math::Vec2(1.0, 0.0)),
            TexturedVertex::new(math::Vec3(-0.5, -0.5, 0.0), math::Vec2(0.0, 1.0)),
            TexturedVertex::new(math::Vec3(0.5, -0.5, 0.0), math::Vec2(1.0, 1.0)),
        ]);
        let shader = sophie
            .wgpu
            .create_shader_from_file(std::path::Path::new("./src/shaders/test3.wgsl"))
            .unwrap();
        sophie.wgpu.add_program_from(
            &shader,
            Some(SophieBufferDataDescriptor {
                layouts: None,
                vertices,
                indices: Some(vec![2, 1, 0, 3, 1, 2]),
                texture: sophie
                    .wgpu
                    .create_texture_from_file(std::path::Path::new("./images/jesus.png"))
                    .ok(),
            }),
        );
        assert_eq!(std::mem::size_of::<[TexturedVertex; 4]>(), (12 + 8) * 4);
        sophie.listen(&sdl, &mut TestHandler);
        Ok(())
    }
}
