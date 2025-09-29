use winit::keyboard::KeyCode;
use cgmath;
use wgpu::util::DeviceExt;

use crate::system;

pub struct CameraBuffer {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl CameraBuffer {
    pub(crate) fn new<C: Camera + ?Sized>(system: &system::System, camera: &C) -> Self {
        // Create a camera buffer to hold the camera matrix
        let buffer = system.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&camera.get_matrix()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        // Create a bind group layout to describe the shader bindings for the camera
        let bind_group_layout = system.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            }
        );

        // Bind the camera buffer to the camera bind group
        let bind_group = system.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            }
        );

        Self { buffer, bind_group_layout, bind_group }
    }
}

pub trait Camera {
    fn buffer(&self) -> &Option<CameraBuffer>;
    fn position(&self) -> cgmath::Vector2<f32>;
    fn interested_keys(&self) -> Vec<KeyCode>;

    fn init_buffer(&mut self, buffer: CameraBuffer);
    
    fn get_matrix(&self) -> [[f32; 4]; 4] {
        cgmath::Matrix4::from_translation(self.position().extend(0.0)).into()
    }

    #[allow(unused_variables)]
    fn key_event(&mut self, key: KeyCode, pressed: bool) {}
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
}