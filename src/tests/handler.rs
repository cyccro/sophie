use wgpu::SurfaceError;

use crate::{
    entities::Entities,
    objects::{
        camera::PerspectiveCamera, drawables::Mesh, SophieKeyboardControllable, UniformUpdateable,
    },
    sophie::{KeydownData, Sophie, SophieHandler},
};

pub enum TestErrors {
    Surface(SurfaceError),
}

pub struct TestHandler1 {
    pub camera: PerspectiveCamera,
    pub entities: Entities,
}

impl SophieHandler<TestErrors> for TestHandler1 {
    fn update(
        &mut self,
        _dt: std::time::Duration,
        sophie: &mut Sophie,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        self.entities.update(sophie.queue(), &mut self.camera);
        match sophie.wgpu.render(&self.meshes()) {
            Ok(_) => crate::sophie::SophieEventResult::Success,
            Err(e) => crate::sophie::SophieEventResult::Error(TestErrors::Surface(e)),
        }
    }
    fn on_keydown(
        &mut self,
        _sophie: &mut Sophie,
        key: KeydownData,
    ) -> crate::sophie::SophieEventResult<TestErrors> {
        self.camera.on_keydown(&key);
        crate::sophie::SophieEventResult::Success
    }
    fn on_keypress(&mut self, sophie: &mut Sophie) -> crate::sophie::SophieEventResult<TestErrors> {
        for key in sophie.pressed_keys().iter() {
            self.camera.on_keydown(&KeydownData {
                scancode: Some(key.clone()),
                kmod: sophie.pressed_keys().modf,
            })
        }
        crate::sophie::SophieEventResult::Success
    }
    fn fallback_error(&mut self, err: TestErrors, sophie: &mut Sophie) {
        match err {
            TestErrors::Surface(e) => {
                println!("{e:#?} eita lasqueira");
            }
        }
        sophie.should_exit();
    }
}

impl TestHandler1 {
    pub fn meshes(&self) -> &Vec<Mesh> {
        self.entities.meshes()
    }
}
