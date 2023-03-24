use wgpu::{CommandEncoder, Device, Queue, RenderPass, RenderPipeline, SurfaceConfiguration, TextureView};
use crate::rendering::model::Model;
use crate::rendering::pipeline::{DefaultPipeline, Pipeline};
use crate::rendering::Shape;

pub trait Drawable {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>, pipeline: &'a RenderPipeline);
}

pub struct Scene {
    drawables: Vec<Box<dyn Drawable>>,
    pipeline: DefaultPipeline,
}

impl Scene {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let drawables = Vec::new();
        let pipeline = DefaultPipeline::create_pipeline(device, config);

        return Self {
            drawables,
            pipeline,
        };
    }

    pub fn draw(&self, encoder: &mut CommandEncoder, view: &TextureView) {
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

        for _drawable in &self.drawables {
            _drawable.draw(&mut render_pass, &self.pipeline.pipeline)
        }
    }

    pub fn add_default_content(&mut self, device: &Device, queue: &Queue) {
        let shape = Shape::load(device, queue, &self.pipeline);

        self.drawables.push(Box::new(shape));
    }
}
