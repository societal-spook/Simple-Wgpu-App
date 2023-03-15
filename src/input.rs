use winit::event::*;
use winit::event_loop::ControlFlow;

pub(crate) struct Input {
    
}

impl Input {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn resolve(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
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