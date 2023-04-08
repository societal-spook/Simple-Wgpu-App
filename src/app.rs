use legion::{Resources, Schedule, World};
use wgpu::SurfaceError;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowId};
use crate::camera;
use crate::camera::{Camera};
use crate::input::Input;
use crate::rendering::renderer::Renderer;

pub struct App {
    pub world: World,
    pub resources: Resources,
    pub schedule: Schedule,
    pub renderer: Renderer,
    input: Input,
}

impl App {
    pub async fn new(window: Window) -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        let mut schedule_builder = Schedule::builder();
        camera::setup(&mut resources, &mut schedule_builder, &window);
        let schedule = schedule_builder.build();

        let mut renderer = Renderer::new(window).await;
        let input = Input::new(&mut resources);

        return Self {
            world,
            resources,
            schedule,
            renderer,
            input,
        };
    }

    pub fn window(&self) -> &Window {
        self.renderer.window()
    }

    pub fn input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.renderer.resize(Some(*physical_size));
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.renderer.resize(Some(**new_inner_size));
            }
            _ => {
                self.input.resolve(&event, control_flow, &mut self.resources);
            }
        }
    }

    pub fn update(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
        self.renderer.window.request_redraw();
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        self.renderer.render(&self.resources, &self.world)
    }
}
