use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::sr_core::helpers::PipelineHelper;

use crate::Vertex;

pub struct SophieProgram {
    pipeline: wgpu::RenderPipeline,
    buffer: Option<wgpu::Buffer>,
    indices: Option<wgpu::Buffer>,
    len: usize,
    size: usize,
    indices_len: usize,
}
pub struct SophieBufferDataDescriptor<'a> {
    pub vertices: Vec<Vertex>,
    pub layouts: Vec<wgpu::VertexBufferLayout<'a>>,
    pub indices: Option<Vec<u16>>,
}
impl SophieProgram {
    pub fn new<'a>(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        buffer_data: Option<SophieBufferDataDescriptor<'a>>,
    ) -> Self {
        if let Some(descriptor) = buffer_data {
            let vertices = descriptor.vertices;
            let layouts = descriptor.layouts;
            let contents = bytemuck::cast_slice(vertices.as_slice());
            Self {
                indices_len: descriptor.indices.as_ref().map(|v| v.len()).unwrap_or(0),
                len: vertices.len(),
                size: std::mem::size_of::<Vertex>() * vertices.len(),
                pipeline: PipelineHelper::create_pipeline(device, shader, Some(layouts)),
                buffer: Some(device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents,
                    usage: wgpu::BufferUsages::VERTEX,
                })),
                indices: descriptor.indices.map(|indices| {
                    device.create_buffer_init(&BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(indices.as_slice()),
                        usage: wgpu::BufferUsages::INDEX,
                    })
                }),
            }
        } else {
            Self {
                indices_len: 0,
                len: 0,
                size: 0,
                buffer: None,
                indices: None,
                pipeline: PipelineHelper::create_pipeline(&device, shader, None),
            }
        }
    }
    pub fn byte_size(&self) -> usize {
        self.size
    }
    pub fn vertices_len(&self) -> usize {
        self.len
    }
    pub fn indices_len(&self) -> usize {
        self.indices_len
    }
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    pub fn index_buffer(&self) -> &Option<wgpu::Buffer> {
        &self.indices
    }
    pub fn buffer(&self) -> &Option<wgpu::Buffer> {
        &self.buffer
    }
}
