use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;
use easy_gpu::{Context, Buffer, PipelineBuilder,
    MatrixStack, RenderPass, Pipeline, MatrixUniform
};

use crate::error::MoonWalkError;
use crate::rendering::vertex::Vertex;

pub struct MoonRenderer {
    context: Context,
    matrix_stack: MatrixStack,
    uniform_buffer: Option<Buffer<MatrixUniform>>,
    bind_group: Option<wgpu::BindGroup>,
    pipeline: Option<Pipeline>,
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
            uniform_buffer: None,
            bind_group: None,
            pipeline: None,
        })
    }

    pub fn new_pipeline(&mut self, shader_source: &str) -> Result<(), MoonWalkError> {
        let uniform_data = self.matrix_stack.to_uniform();
        let uniform_buffer = Buffer::uniform(&self.context, &uniform_data);

        let bind_group_layout = self.context.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            }
        );

        let bind_group = self.context.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.raw.as_entire_binding(),
                }],
            }
        );

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // = 60
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Позиция
                wgpu::VertexAttribute {
                    shader_location: 0,
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                
                // Цвет
                wgpu::VertexAttribute {
                    shader_location: 1,
                    offset: 12,
                    format: wgpu::VertexFormat::Float32x4,
                },

                // Локальная позиция
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: 28,
                    format: wgpu::VertexFormat::Float32x2,
                },

                // Размер
                wgpu::VertexAttribute {
                    shader_location: 3,
                    offset: 36,
                    format: wgpu::VertexFormat::Float32x2,
                },

                // Радиус
                wgpu::VertexAttribute {
                    shader_location: 4,
                    offset: 44,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };

        Ok(())
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
        self.matrix_stack.set_ortho(width as f32, height as f32);
    }
}