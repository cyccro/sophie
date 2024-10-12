use std::process::ExitCode;

use sdl2::event::WindowEvent;

use crate::{errors::SophieResult, sr_core::WgpuState};

use super::{KeydownData, PressedKeys, SophieEventResult, SophieHandler};

pub struct Sophie<'a> {
    id: u32,
    exit: bool,
    pub wgpu: WgpuState<'a>,
    pub pressed_keys: PressedKeys,
    time: std::time::Instant,
}
impl<'a> Sophie<'a> {
    pub async fn new(sdl: &sdl2::Sdl, title: &str, size: (u32, u32)) -> SophieResult<Self> {
        let video = sdl.video().unwrap();
        let window = video
            .window(title, size.0, size.1)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        Ok(Self {
            id: window.id(),
            time: std::time::Instant::now(),
            wgpu: WgpuState::new(&window).await?,
            exit: false,
            pressed_keys: PressedKeys::new(),
        })
    }
    pub fn pressed_keys(&self) -> &PressedKeys {
        &self.pressed_keys
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn device(&self) -> &wgpu::Device {
        self.wgpu.device()
    }
    pub fn queue(&self) -> &wgpu::Queue {
        self.wgpu.queue()
    }
    fn test_handler_result<E>(
        &mut self,
        result: SophieEventResult<E>,
        handler: &mut impl SophieHandler<E>,
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
    pub fn listen<E>(&mut self, sdl: &sdl2::Sdl, handler: &mut impl SophieHandler<E>) -> ExitCode {
        let mut evpump = sdl.event_pump().unwrap();
        let mut window = false;
        loop {
            if self.exit {
                let mut cancel = false;
                handler.before_exit(self, window, &mut cancel);
                if !cancel {
                    return ExitCode::from(1);
                }
                window = false;
            };
            let now = std::time::Instant::now();
            let dt = self.time - now;
            {
                let update = handler.update(dt, self);
                self.test_handler_result(update, handler);
                let keypress = handler.on_keypress(self);
                self.test_handler_result(keypress, handler);
            }
            for event in evpump.poll_iter() {
                match event {
                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Enter => {
                            let result = handler.mouse_enter(self);
                            self.test_handler_result(result, handler)
                        }
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
                    sdl2::event::Event::KeyUp {
                        window_id,
                        scancode,
                        keymod,
                        ..
                    } if window_id == self.id => {
                        self.pressed_keys.modf.remove(keymod);
                        if let Some(scan) = scancode {
                            if self.pressed_keys.is_key_pressed(&scan) {
                                self.pressed_keys.unregister(&scan);
                                let result = handler.on_keyup(
                                    self,
                                    KeydownData {
                                        scancode,
                                        kmod: keymod,
                                    },
                                );
                                self.test_handler_result(result, handler);
                            }
                        }
                    }
                    sdl2::event::Event::KeyDown {
                        window_id,
                        scancode,
                        keymod,
                        ..
                    } if window_id == self.id => {
                        self.pressed_keys.modf = keymod;
                        if let Some(scan) = scancode {
                            if self.pressed_keys.is_key_pressed(&scan) {
                                continue;
                            }
                            self.pressed_keys.register(scan);
                        }
                        let result = handler.on_keydown(
                            self,
                            KeydownData {
                                scancode,
                                kmod: keymod,
                            },
                        );
                        self.test_handler_result(result, handler);
                    }
                    _ => {}
                }
            }
            self.time = std::time::Instant::now();
        }
    }
}
