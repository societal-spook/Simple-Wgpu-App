use legion::{World};
use wgpu::SurfaceError;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowId};
use crate::input::Input;
use crate::renderer::Renderer;

pub struct App {
    world: World,
    pub renderer: Renderer,
    input: Input,
}

impl App {
    pub async fn new(window: Window) -> Self {
        let world = World::default();
        let renderer = Renderer::new(window).await;
        let input = Input::new();

        return Self {
            world,
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
                self.input.resolve(&event, control_flow);
            }
        }
    }

    pub fn update(&mut self) {
        self.renderer.window.request_redraw();
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        self.renderer.render(&self.world)
    }
}