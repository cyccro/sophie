use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::BindGroup;

use crate::sr_core::helpers::{BindGroupInfo, HasBindgroup, PipelineHelper};

use super::Vertices;

pub struct SophieProgram {
    pipeline: wgpu::RenderPipeline,
    buffer: Option<wgpu::Buffer>,
    indices: Option<wgpu::Buffer>,
    len: usize,
    size: usize,
    indices_len: usize,
    bindgroups: Vec<BindGroupInfo>,
}
pub struct SophieBufferDataDescriptor<'a> {
    pub indices: Option<Vec<u16>>,
    pub vertices: Vertices,
    pub attribute_deffinitions: Option<Vec<wgpu::VertexBufferLayout<'a>>>,
    pub info: Vec<BindGroupInfo>,
}
impl<'a> SophieBufferDataDescriptor<'a> {
    pub fn indices_len(&self) -> usize {
        if let Some(ref indices) = self.indices {
            indices.len()
        } else {
            0
        }
    }
    pub fn vertices_len(&self) -> usize {
        self.vertices.len()
    }
}
impl SophieProgram {
    pub fn new<'a>(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        buffer_data: Option<SophieBufferDataDescriptor<'a>>,
    ) -> Self {
        if let Some(descriptor) = buffer_data {
            let indices_len = descriptor.indices_len();
            let attribute_deffinitions = if let Some(ref defs) = descriptor.attribute_deffinitions {
                defs
            } else {
                &vec![descriptor.vertices.layout()]
            };
            let pipeline = PipelineHelper::create_pipeline(
                device,
                shader,
                Some(attribute_deffinitions),
                descriptor.info.as_slice(),
            );
            let contents = descriptor.vertices.as_slice();
            let len = descriptor.vertices.len();
            let size = descriptor.vertices.size();
            Self {
                indices_len,
                bindgroups: descriptor.info,
                pipeline,
                len,
                size,
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
                bindgroups: Vec::new(),
                indices_len: 0,
                len: 0,
                size: 0,
                buffer: None,
                indices: None,
                pipeline: PipelineHelper::create_pipeline(&device, shader, None, &Vec::new()),
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
    pub fn bind_groups(&self) -> &Vec<BindGroupInfo> {
        &self.bindgroups
    }
}
