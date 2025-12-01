#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub local_pos: [f32; 2],
    pub rect_size: [f32; 2],
    pub radii: [f32; 4],
}