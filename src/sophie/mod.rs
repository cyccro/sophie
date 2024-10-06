use std::marker::PhantomData;

use sdl2::event::WindowEvent;

use crate::{math, sr_core::WgpuState};

pub enum SophieEventResult<E> {
    Success,
    Error(E),
    Exit,
}

pub trait SophieHandler<E> {
    fn update(&self, dt: std::time::Duration, sophie: &mut Sophie<E>) -> SophieEventResult<E>;
    fn resize(&self, sophie: &mut Sophie<E>, x: i32, y: i32) -> SophieEventResult<E>;
    fn before_exit(&self, _sophie: &mut Sophie<E>, default_request: bool, cancel: &mut bool) {}
    fn fallback_error(&self, err: E, sophie: &mut Sophie<E>);
}
pub struct Sophie<'a, E> {
    exit: bool,
    pub wgpu: WgpuState<'a>,
    time: std::time::Instant,
    data: PhantomData<E>,
}
impl<'a, E> Sophie<'a, E> {
    pub async fn new(sdl: &sdl2::Sdl, title: &str, size: &math::Vec2) -> Self {
        let video = sdl.video().unwrap();
        let window = video
            .window(title, size.0 as u32, size.1 as u32)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        Self {
            time: std::time::Instant::now(),
            wgpu: WgpuState::new(&window).await,
            exit: false,
            data: PhantomData::default(),
        }
    }
    fn test_handler_result(
        &mut self,
        result: SophieEventResult<E>,
        handler: &impl SophieHandler<E>,
    ) {
        match result {
            SophieEventResult::Exit => self.exit = true,
            SophieEventResult::Error(e) => handler.fallback_error(e, self),
            SophieEventResult::Success => {}
        }
    }
    pub fn should_exit(&mut self) {
        self.exit = true;
    }
    pub fn listen(&mut self, sdl: &sdl2::Sdl, handler: &impl SophieHandler<E>) {
        let mut evpump = sdl.event_pump().unwrap();
        let mut window = false;
        loop {
            if self.exit {
                let mut cancel = false;
                handler.before_exit(self, window, &mut cancel);
                if !cancel {
                    break;
                }
                window = false;
            };
            let now = std::time::Instant::now();
            let dt = self.time - now;
            self.time = now;
            let update = handler.update(dt, self);
            self.test_handler_result(update, handler);
            for event in evpump.poll_iter() {
                match event {
                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(x, y) => {
                            self.wgpu.resize(x as u32, y as u32);
                            let result = handler.resize(self, x, y);
                            self.test_handler_result(result, handler);
                        }
                        WindowEvent::Close => {
                            self.exit = true;
                            window = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
