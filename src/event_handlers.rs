use pixel_canvas::{input::{Event, WindowEvent, glutin::event::{ElementState, MouseButton}}, canvas::CanvasInfo};

use crate::state::ProgramState;


pub fn event_handler(info: &CanvasInfo, program_state: &mut ProgramState, event: &Event<()>) -> bool {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            handle_mouse_move(position, program_state, info)
        },
        Event::WindowEvent { 
            event: WindowEvent::MouseInput { state, button, .. }, 
            .. 
        } => {
            handle_mouse_input(program_state, state, button)
        }
        _ => false,
    }
}

fn handle_mouse_move(position: &pixel_canvas::input::glutin::dpi::PhysicalPosition<f64>, program_state: &mut ProgramState, info: &CanvasInfo) -> bool {
    let (x, y): (i32, i32) = (*position).into();
    program_state.virtual_x = x;
    program_state.virtual_y = y;
    program_state.x = (x as f64 * info.dpi) as i32;
    program_state.y = ((info.height as i32 - y) as f64 * info.dpi) as i32;
    true
}


fn handle_mouse_input(program_state: &mut ProgramState, state: &ElementState, button: &MouseButton) -> bool {
    program_state.mouse_state = *state;

    program_state.mouse_button =  match button {
        MouseButton::Left => MouseButton::Left,
        MouseButton::Right => MouseButton::Right,
        // only allow right or left click, ignore other input
        _ => program_state.mouse_button
    };

    program_state.mouse_click_handled = false;

    true
}
