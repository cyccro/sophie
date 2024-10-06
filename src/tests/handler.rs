use wgpu::SurfaceError;

use crate::sophie::{Sophie, SophieHandler};

pub enum TestErrors {
    Surface(SurfaceError),
}

pub struct TestHandler1;

impl SophieHandler<TestErrors> for TestHandler1 {
    fn update(
        &self,
        dt: std::time::Duration,
        sophie: &mut Sophie<TestErrors>,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        match sophie.wgpu.render() {
            Ok(_) => crate::sophie::SophieEventResult::Success,
            Err(e) => crate::sophie::SophieEventResult::Error(TestErrors::Surface(e)),
        }
    }
    fn resize(
        &self,
        sophie: &mut Sophie<TestErrors>,
        x: i32,
        y: i32,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        println!("{x} {y}");
        crate::sophie::SophieEventResult::Success
    }
    fn fallback_error(&self, err: TestErrors, sophie: &mut Sophie<TestErrors>) {
        match err {
            TestErrors::Surface(e) => {
                println!("{e:#?} eita lasqueira");
            }
        }
        sophie.should_exit();
    }
}
