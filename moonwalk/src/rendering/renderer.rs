use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;
use easy_gpu::{
    Context, Buffer, PipelineBuilder, MatrixStack,
    RenderPass, Pipeline, MatrixUniform,
};

use crate::error::MoonWalkError;
use crate::rendering::vertex::Vertex;
use crate::rendering::shaders::ShaderStore;

pub struct MoonRenderer {
    context: Context,
    matrix_stack: MatrixStack,
    uniform_buffer: Buffer<MatrixUniform>,
    proj_bind_group: wgpu::BindGroup,
    shader_store: ShaderStore,
}

impl MoonRenderer {
    pub fn new(
        window: &'static (impl HasWindowHandle + HasDisplayHandle + Send + Sync),
        width: u32, height: u32
    ) -> Result<Self, MoonWalkError> {
        let window = Arc::new(window);

        let context = pollster::block_on(Context::new(
            window.clone(),
            width,
            height,
        ));

        let mut matrix_stack = MatrixStack::new();
        matrix_stack.set_ortho(context.config.width as f32, context.config.height as f32);

        let uniform_buffer = Buffer::uniform(&context, &matrix_stack.to_uniform());

        let mut shader_store = ShaderStore::new(&context);
        shader_store.create_default_rect(&context, context.config.format)?;
        shader_store.create_default_text(&context, context.config.format)?;
        shader_store.create_default_bezier(&context, context.config.format)?;

        let proj_bind_group = shader_store.get_proj_bind_group(&context, &uniform_buffer);

        Ok(Self {
            context,
            matrix_stack,
            uniform_buffer,
            proj_bind_group,
            shader_store,
        })
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
        self.matrix_stack.set_ortho(width as f32, height as f32);
    }

    pub(crate) fn proj_bind_group(&self) -> &wgpu::BindGroup {
        &self.proj_bind_group
    }
}