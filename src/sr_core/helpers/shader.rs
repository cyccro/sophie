pub struct ShaderHelper;
impl ShaderHelper {
    pub fn create_shader(device: &wgpu::Device, content: &str) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(content.into()),
        })
    }
}
