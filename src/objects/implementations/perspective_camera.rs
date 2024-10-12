use crate::{
    objects::{camera::PerspectiveCamera, SophieKeyboardControllable},
    sophie::KeydownData,
};

impl SophieKeyboardControllable for PerspectiveCamera {
    fn on_keyup(&mut self, data: &KeydownData) {
        println!("Keyup {data:#?}");
    }
    fn on_keydown(&mut self, data: &KeydownData) {
        let Some(code) = data.scancode else {
            return;
        };
        match code {
            sdl2::keyboard::Scancode::A => self.translate_left(0.04),
            sdl2::keyboard::Scancode::W => self.translate_forward(0.04),
            sdl2::keyboard::Scancode::S => self.translate_backward(0.04),
            sdl2::keyboard::Scancode::D => self.translate_right(0.04),
            _ => {}
        }
    }
    fn on_keypress(&mut self, data: &KeydownData) {
        println!("Keypress {data:#?}");
    }
}
