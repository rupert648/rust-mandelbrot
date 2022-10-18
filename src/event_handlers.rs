use pixel_canvas::{
    canvas::CanvasInfo,
    input::{
        glutin::{
            dpi::{PhysicalPosition, PhysicalSize},
            event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode},
        },
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
        Event::WindowEvent {
            event: WindowEvent::MouseInput { state, .. },
            ..
        } => handle_mouse_input(program_state, state),
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => handle_keyboard_input(program_state, input),
        _ => false,
    }
}

fn handle_mouse_move(
    position: &pixel_canvas::input::glutin::dpi::PhysicalPosition<f64>,
    program_state: &mut ProgramState,
    info: &CanvasInfo,
) -> bool {
    let (x, y): (i32, i32) = (*position).into();
    let mouse_state = &mut program_state.mouse_state;
    mouse_state.virtual_x = x;
    mouse_state.virtual_y = y;
    mouse_state.x = (x as f64 * info.dpi) as i32;
    mouse_state.y = ((info.height as i32 - y) as f64 * info.dpi) as i32;

    // if mouse is pressed down, drag camera with mouse
    if mouse_state.element_state == ElementState::Pressed {}

    true
}

fn handle_mouse_input(program_state: &mut ProgramState, state: &ElementState) -> bool {
    // if just pressed, save the position of the mouse for use later
    if program_state.mouse_state.element_state == ElementState::Released
        && *state == ElementState::Pressed
    {
        program_state.camera_state.mouse_start_pos = PhysicalPosition::new(
            program_state.mouse_state.virtual_x,
            program_state.mouse_state.virtual_y,
        );
    }

    program_state.mouse_state.element_state = *state;

    true
}

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
