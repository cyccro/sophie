pub mod camera;
pub mod drawables;
pub mod implementations;
pub mod materials;

use crate::{
    sophie::KeydownData,
    sr_core::{
        helpers::{BindGroupHelper, BufferHelper},
        texture::Texture2D,
    },
};

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

#[derive(Debug)]
pub struct ShaderVar {
    pub group: u32,
    pub binding: u32,
}
#[derive(Debug)]
pub struct ShaderTextureVar {
    pub group: u32,
    pub binding: u32,
    pub texture: Texture2D,
}
#[derive(Debug)]
pub struct ShaderInfo {
    pub vpm: ShaderVar,
    pub data: ShaderVar,
    pub texture: Option<ShaderTextureVar>,
}
impl ShaderVar {
    pub fn new(group: u32, binding: u32) -> Self {
        Self { group, binding }
    }
}
impl ShaderInfo {
    pub fn create_bindgroups(
        &self,
        device: &wgpu::Device,
        vpmbuffer: &wgpu::Buffer,
        matbuffer: &wgpu::Buffer,
    ) -> Vec<(wgpu::BindGroup, wgpu::BindGroupLayout)> {
        let vpm = BindGroupHelper::create_uniform(device, &vpmbuffer, self.vpm.binding);
        let material = BindGroupHelper::create_uniform_vf(device, &matbuffer, self.data.binding);
        return if let Some(ref texture) = self.texture {
            let txt = BindGroupHelper::create_texture(
                device,
                texture.texture.view(),
                texture.texture.sampler(),
                texture.binding,
            );
            let (vpmx, data, txxt) = (
                self.vpm.group as usize,
                self.data.group as usize,
                texture.group as usize,
            );
            let mut indices = vec![vpmx, data, txxt];
            indices.sort_unstable();
            let mut vec = Vec::with_capacity(3);
            if indices[0] == vpmx {
                vec.push(vpm);
                if indices[1] == data {
                    vec.push(material);
                    vec.push(txt);
                } else {
                    vec.push(txt);
                    vec.push(material);
                }
            } else if indices[0] == data {
                vec.push(material);
                if indices[1] == vpmx {
                    vec.push(vpm);
                    vec.push(txt);
                } else {
                    vec.push(txt);
                    vec.push(vpm);
                }
            } else {
                vec.push(txt);
                if indices[1] == vpmx {
                    vec.push(vpm);
                    vec.push(material);
                } else {
                    vec.push(material);
                    vec.push(vpm);
                }
            }
            vec
        } else {
            if self.vpm.group < self.data.group {
                vec![vpm, material]
            } else {
                vec![material, vpm]
            }
        };
    }
}
