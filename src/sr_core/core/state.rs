use std::path::Path;

use crate::{
    errors::{SophieError, SophieResult},
    objects::{
        camera::{PerspectiveCamera, PerspectiveConfigs},
        drawables::Mesh,
    },
    sr_core::{helpers::ShaderHelper, texture::Texture2D, SophieProgram, SophieProgramDescriptor},
};
use wgpu::{
    rwh::{HasDisplayHandle, HasWindowHandle},
    Adapter, Device, Instance, Queue, ShaderModule, Surface, SurfaceError, TextureFormat,
};

pub struct WgpuState<'a> {
    instance: Instance,
    surface: Surface<'a>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    window: sdl2::video::Window,
    programs: Vec<SophieProgram>,
    current_program: Option<SophieProgram>,
}
impl<'a> WgpuState<'a> {
    pub async fn new(window: &sdl2::video::Window) -> SophieResult<Self> {
        let size = window.size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe {
            let Ok(rdh) = window.display_handle() else {
                return Err(SophieError::new(0x1000));
            };
            let Ok(rwh) = window.window_handle() else {
                return Err(SophieError::new(0x1000));
            };
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: rdh.into(),
                raw_window_handle: rwh.into(),
            })
        }
        .map_err(|_| SophieError::new(0x130000))?;
        let Some(adapter) = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
        else {
            return Err(SophieError::new(0x140000));
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("device label"),
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                std::env::var("WGPU_TRACE")
                    .ok()
                    .as_deref()
                    .map(std::path::Path::new),
            )
            .await
            .map_err(|_| SophieError::new(0x150000))?;
        let capabilities = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.0,
            height: size.1,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        Ok(Self {
            window: window.clone(),
            surface,
            config,
            instance,
            adapter,
            queue,
            device,
            current_program: None,
            programs: Vec::new(),
        })
    }
    pub fn device(&self) -> &Device {
        &self.device
    }
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
    pub fn set_program(&mut self, program: SophieProgram) {
        self.current_program = Some(program);
    }
    pub fn set_program_from(
        &mut self,
        shader: &ShaderModule,
        descriptor: Option<SophieProgramDescriptor>,
    ) {
        let program = SophieProgram::new(&self.device, shader, descriptor);
        self.set_program(program);
    }
    pub fn add_program(&mut self, program: SophieProgram) {
        self.programs.push(program);
    }
    pub fn add_program_from(
        &mut self,
        shader: &ShaderModule,
        descriptor: Option<SophieProgramDescriptor>,
    ) {
        self.programs
            .push(SophieProgram::new(&self.device, shader, descriptor));
    }
    pub fn add_program_from_source(
        &mut self,
        shader: &str,
        descriptor: Option<SophieProgramDescriptor>,
    ) {
        self.add_program_from(
            &ShaderHelper::create_shader(&self.device, shader),
            descriptor,
        );
    }
    pub fn create_shader_from_file(&self, path: &std::path::Path) -> SophieResult<ShaderModule> {
        ShaderHelper::create_from_file(&self.device, path)
    }
    pub fn create_texture(&self, bytes: &[u8]) -> SophieResult<Texture2D> {
        Texture2D::new(&self.queue, &self.device, bytes)
    }
    pub fn create_texture_from_file(&self, path: &Path) -> SophieResult<Texture2D> {
        Texture2D::from_file(&self.queue, &self.device, path)
    }
    pub async fn create_texture_async(&self, path: &Path) -> SophieResult<Texture2D> {
        Texture2D::from_file_async(&self.queue, &self.device, path).await
    }
    pub fn create_perspective_camera(
        &self,
        position: na::Point3<f32>,
        target: na::Point3<f32>,
        fov: f32,
        far: f32,
        near: f32,
    ) -> PerspectiveCamera {
        PerspectiveCamera::new(
            target,
            position,
            PerspectiveConfigs::new(
                fov,
                far,
                near,
                self.config.width as f32 / self.config.height as f32,
            ),
        )
    }
    pub fn render(&self, meshes: &Vec<Mesh>) -> Result<(), SurfaceError> {
        let txt = self.surface.get_current_texture()?;
        let view = txt
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        //make rpass go out of scope to encoder finish be able
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            if let Some(ref program) = self.current_program {
                let groups = program.bind_groups();
                rpass.set_pipeline(program.pipeline());
                for (idx, bindgroup) in groups.iter().enumerate() {
                    rpass.set_bind_group(idx as u32, bindgroup.group(), &[]);
                }
                for mesh in meshes {
                    mesh.draw((groups.len()) as u32, &mut rpass);
                }
            }
        }
        let finish = encoder.finish();
        self.queue.submit(Some(finish));
        txt.present();
        Ok(())
    }
    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        if width + height != 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
