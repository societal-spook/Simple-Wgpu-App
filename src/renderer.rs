use legion::{Resources, Schedule, World};
use winit::window::Window;
use crate::core::Core;

pub struct Renderer {
    core: Core,
    pub(crate) window: Window,
}

impl Renderer {
    pub async fn new(window: Window) -> Self {
        let core = Core::new(&window).await;

        Self {
            core,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn render(&mut self, world: &World) -> Result<(), wgpu::SurfaceError> {
        self.core.render()
    }

    pub fn resize(&mut self, new_size: Option<winit::dpi::PhysicalSize<u32>>) {
        self.core.resize(new_size);
    }
}
