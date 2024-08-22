use bytemuck;
use nalgebra::Point3;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::NoUninit, bytemuck::Zeroable)]
pub struct Vertex {
    // todo: I think that 3D points in graphics are Point4s since there's some mathy thing going on
    pub position: Point3<f32>,
    pub color: Point3<f32>,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
