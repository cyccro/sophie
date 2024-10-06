use sophie::Sophie;
use sr_core::{SophieBufferDataDescriptor, Vertex, VertexLayout};
use std::io::Read;
use tests::TestHandler1;
use tracing_subscriber;
mod math;
mod sophie;
mod sr_core;
mod tests;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::init();

    let sdl = sdl2::init().unwrap();
    let mut sophie = Sophie::new(&sdl, "Sophie Window", &math::Vec2(1080.0, 720.0)).await;
    let shader = {
        let mut buffer = String::new();
        std::fs::File::open("./src/shaders/test2.wgsl")
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        buffer
    };
    let vertices = vec![
        Vertex::new(math::Vec3(-0.5, 0.5, 0.0), math::Rgba(0.0, 1.0, 0.0, 1.0)),
        Vertex::new(math::Vec3(0.5, 0.5, 0.0), math::Rgba(1.0, 0.0, 0.0, 1.0)),
        Vertex::new(math::Vec3(-0.5, -0.5, 0.0), math::Rgba(0.0, 0.0, 1.0, 1.0)),
        Vertex::new(math::Vec3(0.5, -0.5, 0.0), math::Rgba(1.0, 1.0, 1.0, 1.0)),
    ];
    sophie.wgpu.add_program_from_source(
        &*shader,
        Some(SophieBufferDataDescriptor {
            layouts: vec![Vertex::layout()],
            vertices,
            indices: Some(vec![2, 1, 0, 3, 1, 2]),
        }),
    );
    sophie.listen(&sdl, &TestHandler1);
}

#[cfg(test)]
mod testing {
    use std::io::Read;

    use crate::{
        math,
        sophie::{SophieEventResult, SophieHandler},
        wgpu::sr_core::{SophieBufferDataDescriptor, Vertex, VertexLayout},
        Sophie,
    };
    #[tokio::test]
    async fn draw_square() -> Result<(), ()> {
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
        let sdl = sdl2::init().map_err(|_| ())?;
        let handler = TestHandler;
        let mut sophie = Sophie::new(&sdl, "Window From Sophie", &math::Vec2(1080.0, 720.0)).await;
        let shader = {
            let mut buffer = String::new();
            std::fs::File::open("./src/shaders/test2.wgsl")
                .unwrap()
                .read_to_string(&mut buffer)
                .unwrap();
            buffer
        };
        let vertices = vec![
            Vertex::new(math::Vec3(-0.5, 0.5, 0.0), math::Rgba(0.0, 1.0, 0.0, 1.0)),
            Vertex::new(math::Vec3(0.5, 0.5, 0.0), math::Rgba(1.0, 0.0, 0.0, 1.0)),
            Vertex::new(math::Vec3(-0.5, -0.5, 0.0), math::Rgba(0.0, 0.0, 1.0, 1.0)),
            Vertex::new(math::Vec3(0.5, -0.5, 0.0), math::Rgba(1.0, 1.0, 1.0, 1.0)),
        ];
        sophie.wgpu.add_program_from_source(
            &*shader,
            Some(SophieBufferDataDescriptor {
                layouts: vec![Vertex::layout()],
                vertices,
                indices: Some(vec![2, 1, 0, 3, 1, 2]),
            }),
        );
        assert_eq!(std::mem::size_of::<[Vertex; 4]>(), (12 + 16) * 4);
        sophie.listen(&sdl, &handler);
        Ok(())
    }
}
