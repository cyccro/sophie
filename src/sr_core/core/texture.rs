use std::path::Path;

use image::GenericImageView;
use wgpu::{
    BindGroup, Device, Extent3d, Queue, RenderPass, Sampler, Texture, TextureDescriptor,
    TextureView,
};

use crate::{
    errors::{SophieError, SophieResult},
    sr_core::helpers::{BindGroupInfo, BindGroupKind, HasBindgroup},
};

#[derive(Debug)]
pub struct Texture2D {
    width: u32,
    height: u32,
    view: TextureView,
    wgpu_texture: Texture,
    pub info: BindGroupInfo,
    sampler: Sampler,
}

impl Texture2D {
    pub fn from_file(queue: &Queue, device: &Device, path: &Path) -> SophieResult<Self> {
        let data = std::fs::read(path).map_err(|_| SophieError::new(0x100200))?;
        Self::new(queue, device, data.as_slice())
    }
    pub async fn from_file_async(
        queue: &Queue,
        device: &Device,
        path: &Path,
    ) -> SophieResult<Self> {
        let data = tokio::fs::read(path)
            .await
            .map_err(|_| SophieError::new(0x100200))?;
        Self::new(queue, device, data.as_slice())
    }
    pub fn new(queue: &Queue, device: &Device, bytes: &[u8]) -> SophieResult<Self> {
        let img = image::load_from_memory(bytes).map_err(|_| SophieError::new(0x100202))?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let wgpu_texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: texture_size,
            dimension: wgpu::TextureDimension::D2,
            mip_level_count: 1,
            sample_count: 1,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &wgpu_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );
        let view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let info = BindGroupInfo::new(device, &BindGroupKind::TEXTURE(&view, &sampler));
        Ok(Self {
            view,
            sampler,
            wgpu_texture,
            width: dimensions.0,
            height: dimensions.1,
            info,
        })
    }

    pub fn view(&self) -> &TextureView {
        &self.view
    }
    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
    pub fn bind_group(&self) -> &BindGroup {
        self.info.group()
    }
    pub fn bind(&self, rpass: &mut RenderPass) {
        rpass.set_bind_group(0, &self.info.group(), &[]);
    }
}
impl HasBindgroup for Texture2D {
    fn info(&self, device: &Device) -> BindGroupInfo {
        BindGroupInfo::new(
            device,
            &BindGroupKind::TEXTURE(&self.view(), self.sampler()),
        )
    }
}
