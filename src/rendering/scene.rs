use std::ops::Deref;
use wgpu::{CommandEncoder, Device, Queue, RenderPass, RenderPipeline, SurfaceConfiguration, TextureView};
use winit::dpi::{PhysicalSize, Size};
use crate::rendering::model::Model;
use crate::rendering::pipeline::{DefaultPipeline, Pipeline};
use crate::rendering::Shape;
use legion::{IntoQuery, query, Query, Resources, World};
use legion::query::EntityFilter;
use crate::camera::Camera;
use crate::rendering::camera::GpuCamera;

pub trait Drawable {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>, pipeline: &'a RenderPipeline);
}

pub struct Scene {
    drawables: Vec<Box<dyn Drawable>>,
    pipeline: DefaultPipeline,
    camera: GpuCamera,
}

impl Scene {
    pub fn new(device: &Device, config: &SurfaceConfiguration, size: &PhysicalSize<u32>) -> Self {
        let drawables = Vec::new();
        let pipeline = DefaultPipeline::create_pipeline(device, config);
        let camera = GpuCamera::load(&device, pipeline.get_camera_layout());

        return Self {
            drawables,
            pipeline,
            camera,
        };
    }

    pub fn draw(&mut self, queue: &Queue, encoder: &mut CommandEncoder, view: &TextureView, resources: &Resources, world: &World) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        let camera = resources.get::<Camera>();
        self.camera.update(queue, camera.unwrap().deref());

        render_pass.set_bind_group(1, &self.camera.camera_bind_group, &[]);

        for _drawable in &self.drawables {
            _drawable.draw(&mut render_pass, &self.pipeline.pipeline)
        }
    }

    pub fn add_default_content(&mut self, device: &Device, queue: &Queue) {
        let shape = Shape::load(device, queue, &self.pipeline);

        self.drawables.push(Box::new(shape));
    }
}
