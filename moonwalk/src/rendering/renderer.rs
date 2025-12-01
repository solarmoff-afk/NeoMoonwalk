use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use easy_gpu::{Context, Buffer, PipelineBuilder, MatrixStack, RenderPass};
use std::sync::Arc;

use crate::error::MoonWalkError;

pub struct MoonRenderer {
    context: Context,
    matrix_stack: MatrixStack,
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

        Ok(Self {
            context,
            matrix_stack,
        })
    }
}