use crate::sr_core::helpers::{BindGroupInfoKind, PipelineHelper};

pub struct SophieProgram {
    pipeline: wgpu::RenderPipeline,
}
pub struct SophieProgramDescriptor<'a> {
    pub attribute_deffinitions: Option<Vec<wgpu::VertexBufferLayout<'a>>>,
    pub groups: Vec<BindGroupInfoKind>,
}
impl SophieProgram {
    pub fn new<'a>(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        buffer_data: Option<&SophieProgramDescriptor<'a>>,
    ) -> Self {
        if let Some(descriptor) = buffer_data {
            let pipeline = PipelineHelper::create_pipeline(
                device,
                shader,
                descriptor.attribute_deffinitions.as_ref(),
                &descriptor.groups,
            );
            Self { pipeline }
        } else {
            Self {
                pipeline: PipelineHelper::create_pipeline(&device, shader, None, &Vec::new()),
            }
        }
    }
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
