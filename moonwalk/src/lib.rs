use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use glam::Vec4;
use wgpu::SurfaceError;

pub mod error;

pub struct MoonWalk {
    test: bool,
}

impl MoonWalk {
    pub fn new(
        window: &'static (impl HasWindowHandle + HasDisplayHandle + Send + Sync),
        width: u32,
        height: u32,
    ) -> Result<Self, error::MoonWalkError> {
        Ok(Self {
            test: true,
        })
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        
    }

    pub fn render_frame(&mut self, clear_color: Vec4) -> Result<(), SurfaceError> {
        Ok(())
    }
}