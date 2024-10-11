use wgpu::SurfaceError;

use crate::{
    objects::{camera::PerspectiveCamera, UniformUpdateable},
    sophie::{Sophie, SophieHandler},
};

pub enum TestErrors {
    Surface(SurfaceError),
}

pub struct TestHandler1 {
    pub camera: PerspectiveCamera,
    pub objects: Vec<Box<dyn UniformUpdateable>>,
}

impl SophieHandler<TestErrors> for TestHandler1 {
    fn update(
        &mut self,
        dt: std::time::Duration,
        sophie: &mut Sophie<TestErrors>,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        //self.camera.translate_nums(0.004, 0.01, 0.0);
        self.camera.update(sophie.wgpu.queue());
        match sophie.wgpu.render() {
            Ok(_) => crate::sophie::SophieEventResult::Success,
            Err(e) => crate::sophie::SophieEventResult::Error(TestErrors::Surface(e)),
        }
    }
    fn mouse_enter(
        &mut self,
        sophie: &mut Sophie<TestErrors>,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        crate::sophie::SophieEventResult::Success
    }
    fn resize(
        &mut self,
        sophie: &mut Sophie<TestErrors>,
        x: i32,
        y: i32,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        crate::sophie::SophieEventResult::Success
    }
    fn fallback_error(&mut self, err: TestErrors, sophie: &mut Sophie<TestErrors>) {
        match err {
            TestErrors::Surface(e) => {
                println!("{e:#?} eita lasqueira");
            }
        }
        sophie.should_exit();
    }
}
