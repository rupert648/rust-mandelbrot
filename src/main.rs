mod event_handlers;
mod state;

use pixel_canvas::input::glutin::event::{MouseButton};
use pixel_canvas::{Canvas, Color};

use crate::event_handlers::event_handler;
use crate::state::ProgramState;

fn main() {
    let width = 1200;
    let height = 900;
    let max_iterations = 1000;

    let canvas = Canvas::new(width as usize, height as usize)
        .title("Mandelbrot")
        .state(ProgramState::new())
        .input(event_handler);

    let mut current_iterations = 10;
    let mut pixels = calculate_mandelbrot(current_iterations, width, height);
    
    canvas.render(move |state, image| {
        // check if we have handled the most recent mouse click
        if !state.mouse_click_handled {
            mouse_clicked(state, &mut current_iterations, max_iterations, &mut pixels, width, height);
            state.mouse_click_handled = true
        }
        
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = pixels[x][y];
            }
        }

    });
    
}

fn mouse_clicked(state: &mut ProgramState, current_iterations: &mut u32, max_iterations: u32, pixels: &mut Vec<Vec<Color>>, width: u32, height: u32) {
    let click_change_amount = 10;
    // right click increases iterations
    // left click decreases iteration
    match state.mouse_button {
        MouseButton::Left => *current_iterations = if *current_iterations <= click_change_amount { click_change_amount } else { *current_iterations-click_change_amount },
        MouseButton::Right => *current_iterations = if *current_iterations >= max_iterations { max_iterations } else { *current_iterations+click_change_amount },
        _ => ()
    }
    // recalculate the mandelbrot based upon the new iteration amount
    *pixels = calculate_mandelbrot(*current_iterations, width, height);
}


fn calculate_mandelbrot(max_iterations: u32, width_u32: u32, height_u32: u32) -> Vec<Vec<Color>> {
    let default_color = Color {
        r: 0,
        g: 0,
        b: 0
    };

    let mut pixels = vec![vec![default_color; height_u32 as usize]; width_u32 as usize];

    let width: f32 = width_u32 as f32;
    let height: f32 = height_u32 as f32;
    
    let w: f32  = 5.0;
    let h: f32 = (w * height) / width;

    let xmin: f32 = -w / 2.0;
    let ymin: f32= -h / 2.0;

    let xmax: f32 = xmin + w;
    let ymax: f32= ymin + h;

    // calculate amount we increment x, y for each pixel
    let dx: f32 = (xmax - xmin) / width;
    let dy: f32= (ymax - ymin) / height;

    let mut y: f32 = ymin;
    for j in 0..height_u32 {
        let mut x: f32 = xmin;
        for i in 0..width_u32 {

            let mut a = x;
            let mut b = y;
            let mut n = 0;
            while n < max_iterations {
                let a_squared: f32 = a * a;
                let b_squared: f32 = b * b;
                let two_ab: f32 = 2.0 * a * b;

                a = a_squared - b_squared + x;
                b = two_ab + y;
                if a*a + b*b > 16.0 {
                    break;
                }
                n += 1;
            }


            if n != max_iterations {
                let val = f32::sqrt(n as f32 / max_iterations as f32);
                let color_val = (val * 255.0) as u8;
                let color = Color {
                    r: color_val,
                    g: color_val,
                    b: color_val
                };

                pixels[i as usize][j as usize] = color;

            }

            x += dx;

        }
        y += dy;
    }

    pixels
}



