use legion::Resources;
use winit::event::*;
use winit::event_loop::ControlFlow;

pub(crate) struct Input {
    
}

pub struct Actions {

}

impl Actions {
    fn default() -> Self {
        Actions{}
    }
}

impl Input {
    pub fn new(resources: &mut Resources) -> Self {
        resources.insert(Actions::default());

        return Self {};
    }

    pub fn resolve(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow, resources: &mut Resources) {
        let actions = resources.get::<Actions>();

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    }
}
