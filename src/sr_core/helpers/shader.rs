use std::io::Read;

use wgpu::Device;

use crate::{errors::SophieError, sophie::SophieResult};

pub struct ShaderHelper;
impl ShaderHelper {
    pub fn create_shader(device: &wgpu::Device, content: &str) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(content.into()),
        })
    }
    pub fn create_from_file(
        device: &Device,
        path: &std::path::Path,
    ) -> SophieResult<wgpu::ShaderModule> {
        let mut buffer = String::new();
        let Ok(mut file) = std::fs::File::open(path) else {
            return Err(SophieError::new(0x1010));
        };
        if let Err(_) = file.read_to_string(&mut buffer) {
            return Err(SophieError::new(0x1011));
        }
        Ok(Self::create_shader(device, &*buffer))
    }
}
