use crate::sr_core::helpers::{BindGroupInfo, BindGroupInfoKind, BindGroupKind, PipelineHelper};

pub struct SophieProgram {
    pipeline: wgpu::RenderPipeline,
    bindgroups: Vec<BindGroupInfo>,
}
pub struct SophieProgramDescriptor<'a> {
    pub attribute_deffinitions: Option<Vec<wgpu::VertexBufferLayout<'a>>>,
    pub info: Vec<BindGroupInfo>,
    pub kinds: Vec<BindGroupInfoKind>,
}
impl SophieProgram {
    pub fn new<'a>(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        buffer_data: Option<SophieProgramDescriptor<'a>>,
    ) -> Self {
        if let Some(descriptor) = buffer_data {
            let attribute_deffinitions = if let Some(ref defs) = descriptor.attribute_deffinitions {
                defs
            } else {
                &vec![]
            };
            let pipeline = PipelineHelper::create_pipeline(
                device,
                shader,
                Some(attribute_deffinitions),
                &descriptor.kinds,
            );
            Self {
                pipeline,
                bindgroups: descriptor.info,
            }
        } else {
            Self {
                pipeline: PipelineHelper::create_pipeline(&device, shader, None, &Vec::new()),
                bindgroups: Vec::new(),
            }
        }
    }
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    pub fn bind_groups(&self) -> &Vec<BindGroupInfo> {
        &self.bindgroups
    }
}
