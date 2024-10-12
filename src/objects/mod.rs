pub mod camera;
pub mod drawables;
pub mod implementations;
use wgpu::Queue;

use crate::sophie::KeydownData;

pub trait UniformUpdateable {
    fn update(&mut self, queue: &Queue);
}
pub trait SophieKeyboardControllable {
    fn on_keydown(&mut self, data: &KeydownData);
    fn on_keyup(&mut self, data: &KeydownData);
    fn on_keypress(&mut self, data: &KeydownData);
}
pub trait SophieMouseControllable {
    fn on_mousedown(&mut self);
    fn on_mouseup(&mut self);
    fn on_mousepress(&mut self);
}
