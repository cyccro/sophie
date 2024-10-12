use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Sampler, TextureView};

#[derive(Debug)]
pub enum BindGroupKind<'a> {
    TEXTURE(&'a TextureView, &'a Sampler),
    UNIFORM(&'a Buffer),
}
#[derive(Debug)]
pub enum BindGroupInfoKind {
    Texture,
    Uniform,
}
pub trait HasBindgroup {
    fn info(&self, device: &Device) -> BindGroupInfo;
}
#[derive(Debug)]
pub struct BindGroupInfo {
    pub layout: BindGroupLayout,
    pub group: BindGroup,
}
impl BindGroupInfo {
    pub fn new(device: &Device, kind: &BindGroupKind) -> Self {
        let (layout, group) = match kind {
            BindGroupKind::TEXTURE(txt_view, sampler) => {
                let layout = BindGroupHelper::create_layout_texture(device);
                let group = BindGroupHelper::create_texture(device, txt_view, sampler);
                (layout, group)
            }
            BindGroupKind::UNIFORM(buffer) => {
                let layout = BindGroupHelper::create_uniform_layout(device);
                let group = BindGroupHelper::create_uniform(device, buffer);
                (layout, group)
            }
        };
        Self { layout, group }
    }
    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }
    pub fn group(&self) -> &BindGroup {
        &self.group
    }
}
pub struct BindGroupHelper;

impl BindGroupHelper {
    pub fn create_texture(device: &Device, txt_view: &TextureView, sampler: &Sampler) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &Self::create_layout_texture(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(txt_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }
    pub fn create_layout_texture(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
    pub fn create_uniform(device: &Device, buffer: &Buffer) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::create_uniform_layout(device),
            label: None,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }
    pub fn create_uniform_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        })
    }
}
