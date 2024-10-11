use crate::objects::{camera::PerspectiveCamera, SophieKeyboardControllable};

impl SophieKeyboardControllable for PerspectiveCamera {
    fn on_keyup(&mut self) {
        println!("Keyup");
    }
    fn on_keydown(&mut self) {
        println!("Keydown");
    }
    fn on_keypress(&mut self) {
        println!("Keypress");
    }
}
