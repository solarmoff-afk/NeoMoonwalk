use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use easy_gpu::{Context, Buffer, PipelineBuilder, MatrixStack, RenderPass};
use std::sync::Arc;

use crate::error::MoonWalkError;

pub struct MoonRenderer {
    context: Context
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

        Ok(Self {
            context,
        })
    }
}