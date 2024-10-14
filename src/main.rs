extern crate nalgebra as na;

use std::process::ExitCode;

use entities::Entities;
use math::{Rgba, Transform, Vec3};
use na::Point3;
use objects::{drawables::Mesh, ShaderInfo, ShaderVar};
use sophie::Sophie;
use sr_core::{
    helpers::BindGroupInfoKind, SophieProgramDescriptor, TexturedVertex, Vertex, Vertices,
};
use tests::TestHandler1;
mod entities;
mod errors;
mod math;
mod objects;
mod sophie;
mod sr_core;
mod tests;

#[tokio::main]
async fn main() -> ExitCode {
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
    let vertices = Vertices::Vertex(vec![
        Vertex::new(Vec3(-1.0, -1.0, -1.0), Rgba(1.0, 1.0, 0.0, 1.0)),
        Vertex::new(Vec3(-1.0, -1.0, 1.0), Rgba(1.0, 1.0, 0.0, 1.0)),
        Vertex::new(Vec3(-1.0, 1.0, -1.0), Rgba(0.0, 1.0, 1.0, 1.0)),
        Vertex::new(Vec3(-1.0, 1.0, 1.0), Rgba(1.0, 0.0, 0.0, 1.0)),
        Vertex::new(Vec3(1.0, -1.0, -1.0), Rgba(0.0, 1.0, 0.0, 1.0)),
        Vertex::new(Vec3(1.0, -1.0, 1.0), Rgba(0.0, 0.0, 1.0, 1.0)),
        Vertex::new(Vec3(1.0, 1.0, -1.0), Rgba(0.3, 0.7, 1.0, 1.0)),
        Vertex::new(Vec3(1.0, 1.0, 1.0), Rgba(1.0, 0.5, 0.2, 1.0)),
    ]);
    let layout = vertices.layout();
    let camera = sophie.wgpu.create_perspective_camera(
        Point3::new(0.0, 2.0, 10.0),
        Point3::origin(),
        120.0,
        1000.0,
        0.1,
    );
    let descriptor = SophieProgramDescriptor {
        attribute_deffinitions: Some(vec![layout]),
        groups: vec![
            BindGroupInfoKind::Uniform(0),
            BindGroupInfoKind::VfUniform(0),
        ],
    };
    sophie
        .wgpu
        .add_program_from("shader", &shader, Some(&descriptor));
    let mut mesh = Mesh::new(
        sophie.device(),
        Transform::default(),
        vertices,
        vec![
            1, 5, 7, 1, 7, 3, // Back face
            0, 2, 6, 0, 6, 4, // Left face
            0, 1, 3, 0, 3, 2, // Right face
            4, 6, 7, 4, 7, 5, // Top face
            2, 3, 7, 2, 7, 6, // Bottom face
            0, 4, 5, 0, 5, 1,
        ],
        ShaderInfo {
            vpm: ShaderVar {
                group: 0,
                binding: 0,
            },
            data: ShaderVar {
                group: 1,
                binding: 0,
            },
            texture: None,
        },
        "shader",
    );
    mesh.translate(&na::Vector3::new(10.0, 0.0, 0.0));
    sophie.listen(
        &sdl,
        &mut TestHandler1 {
            camera,
            entities: Entities::with_meshes(vec![mesh]),
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
        sr_core::{
            helpers::HasBindgroup, SophieProgramDescriptor, TexturedVertex, Vertex, Vertices,
        },
        tests::TestHandler1,
        Sophie,
    };
    use na::Point3;
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
    async fn draw_sprite() -> Result<(), ()> {
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
            Point3::new(0.0, 1.0, 1.0),
            Point3::origin(),
            40.0,
            1000.0,
            0.1,
        );
        sophie.wgpu.add_program_from(
            &shader,
            Some(SophieProgramDescriptor {
                attribute_deffinitions: None,
                vertices,
                info: vec![img.info(sophie.device()), camera.info(sophie.device())],
                indices: Some(vec![2, 1, 0, 3, 1, 2]),
            }),
        );
        sophie.listen(
            &sdl,
            &mut TestHandler1 {
                camera,
                objects: vec![],
            },
        );
        Ok(())
    }
}
