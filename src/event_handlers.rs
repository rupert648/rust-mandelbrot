use pixel_canvas::{
    canvas::CanvasInfo,
    input::{
        glutin::event::{ElementState, KeyboardInput, VirtualKeyCode},
        Event, WindowEvent,
    },
};

use crate::state::{MandelbrotState, ProgramState};

pub fn event_handler(
    info: &CanvasInfo,
    program_state: &mut ProgramState,
    event: &Event<()>,
) -> bool {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => handle_mouse_move(position, program_state, info),
        // Event::WindowEvent {
        //     event: WindowEvent::MouseInput { state, button, .. },
        //     ..
        // }
        // => handle_mouse_input(program_state, state, button),
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => handle_keyboard_input(program_state, input),
        Event::NewEvents(_) => todo!(),
        _ => false,
    }
}

fn handle_mouse_move(
    position: &pixel_canvas::input::glutin::dpi::PhysicalPosition<f64>,
    program_state: &mut ProgramState,
    info: &CanvasInfo,
) -> bool {
    let (x, y): (i32, i32) = (*position).into();
    program_state.virtual_x = x;
    program_state.virtual_y = y;
    program_state.x = (x as f64 * info.dpi) as i32;
    program_state.y = ((info.height as i32 - y) as f64 * info.dpi) as i32;
    true
}

// fn handle_mouse_input(
//     program_state: &mut ProgramState,
//     state: &ElementState,
//     button: &MouseButton,
// ) -> bool {
//     program_state.mouse_state = *state;

//     program_state.mouse_button = match button {
//         MouseButton::Left => MouseButton::Left,
//         MouseButton::Right => MouseButton::Right,
//         // only allow right or left click, ignore other input
//         _ => program_state.mouse_button,
//     };
//     true
// }

fn handle_keyboard_input(program_state: &mut ProgramState, input: &KeyboardInput) -> bool {
    // only handle input on release
    if input.state == ElementState::Pressed {
        return false;
    }

    match input.virtual_keycode {
        Some(x) => match x {
            VirtualKeyCode::A => decrease_iterations(program_state),
            VirtualKeyCode::D => increase_iterations(program_state),
            _ => (),
        },
        None => (),
    };

    true
}

fn increase_iterations(program_state: &mut ProgramState) {
    let mandelbrot_state: &mut MandelbrotState = &mut program_state.mandelbrot_state;

    let mut current_iterations = mandelbrot_state.iterations;
    let max_iterations = mandelbrot_state.max_iterations;
    let click_change_amount = mandelbrot_state.click_change_amount;

    current_iterations = if current_iterations >= max_iterations {
        max_iterations
    } else {
        current_iterations + click_change_amount
    };

    mandelbrot_state.iterations = current_iterations;
    program_state.mandelbrot_state_modified = true;
}

fn decrease_iterations(program_state: &mut ProgramState) {
    let mandelbrot_state = &mut program_state.mandelbrot_state;

    let mut current_iterations = mandelbrot_state.iterations;
    let click_change_amount = mandelbrot_state.click_change_amount;

    current_iterations = if current_iterations <= click_change_amount {
        click_change_amount
    } else {
        current_iterations - click_change_amount
    };

    mandelbrot_state.iterations = current_iterations;
    program_state.mandelbrot_state_modified = true;
}
