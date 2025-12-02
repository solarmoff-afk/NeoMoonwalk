pub mod error;
mod rendering;
mod objects;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use glam::Vec4;
use wgpu::SurfaceError;
use easy_gpu::{Context, Buffer, PipelineBuilder, MatrixStack, RenderPass};
use std::sync::Arc;

use crate::rendering::renderer::MoonRenderer;

pub struct MoonWalk {
    renderer: MoonRenderer,
}

impl MoonWalk {
    pub fn new(
        window: &'static (impl HasWindowHandle + HasDisplayHandle + Send + Sync),
        width: u32,
        height: u32,
    ) -> Result<Self, error::MoonWalkError> {
        let renderer_result = MoonRenderer::new(
            window,
            width,
            height,
        );

        let renderer = match renderer_result {
            Ok(r) => r,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Self {
            renderer,
        })
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.renderer.set_viewport(width, height);
    }

    pub fn render_frame(&mut self, clear_color: Vec4) -> Result<(), SurfaceError> {
        Ok(())
    }
}