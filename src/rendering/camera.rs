use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue};
use wgpu::util::DeviceExt;
use crate::camera::Camera;
use crate::rendering::pipeline::Pipeline;

pub struct GpuCamera {
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
}

impl GpuCamera {
    pub fn update(&mut self, queue: &Queue, camera: &Camera) {
        self.camera_uniform.update_view_proj(camera);

        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn load(device: &Device, camera_bind_group_layout: &BindGroupLayout) -> Self {
        let mut camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        Self {
            camera_uniform,
            camera_buffer,
            camera_bind_group,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
