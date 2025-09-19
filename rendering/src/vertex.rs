// Struct to describe vertex data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) tex_coords: [f32; 2],
}

impl Vertex {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        // A description of how the data is structured in the vertex buffer
        wgpu::VertexBufferLayout {
            // Size of the data for each vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Whether each element of the array represents per-vertex data or per-instance data
            step_mode: wgpu::VertexStepMode::Vertex,
            // Structure of each vertexes' data
            attributes: &[
                wgpu::VertexAttribute {
                    // First attribute starts at the beginning of the vertex data
                    offset: 0,
                    // This attribute will correspond to @location(0) in the shader
                    shader_location: 0,
                    // Format as Vec3<f32>
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // Second attribute starts with an offset equal to the size of the first attribute
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    // This attribute will correspond to @location(1) in the shader
                    shader_location: 1,
                    // Format as Vec3<f32>
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}