use std::collections::HashSet;

use sdl2::keyboard::{Mod, Scancode};

#[derive(Debug)]
pub struct KeydownData {
    pub scancode: Option<Scancode>,
    pub kmod: Mod,
}
pub struct PressedKeys {
    keys: HashSet<sdl2::keyboard::Scancode>,
    pub modf: Mod,
}

impl PressedKeys {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
            modf: Mod::empty(),
        }
    }
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Scancode> {
        self.keys.iter()
    }
    pub fn keys(&self) -> &HashSet<Scancode> {
        &self.keys
    }
    pub fn is_key_pressed(&self, scan: &Scancode) -> bool {
        self.keys.get(scan).is_some()
    }
    pub fn register(&mut self, scan: Scancode) {
        self.keys.insert(scan);
    }
    pub fn unregister(&mut self, scan: &Scancode) {
        self.keys.remove(scan);
    }
}
