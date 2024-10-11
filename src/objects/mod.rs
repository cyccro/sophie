pub mod camera;
pub mod implementations;
use wgpu::Queue;

use crate::sophie::SophieEventResult;

pub trait UniformUpdateable {
    fn update(&mut self, queue: &Queue);
}
pub trait SophieKeyboardControllable {
    fn on_keydown(&mut self);
    fn on_keyup(&mut self);
    fn on_keypress(&mut self);
}
pub trait SophieMouseControllable {
    fn on_mousedown(&mut self);
    fn on_mouseup(&mut self);
    fn on_mousepress(&mut self);
}
