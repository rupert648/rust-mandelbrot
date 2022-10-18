mod config;
mod event_handlers;
mod state;

use config::Config;
use pixel_canvas::{Canvas, Color};

use crate::event_handlers::event_handler;
use crate::state::ProgramState;

fn main() -> Result<(), ::std::io::Error> {
    let cfg: Config = confy::load("mandelbrot")?;

    let width = cfg.width;
    let height = cfg.height;

    let canvas = Canvas::new(width as usize, height as usize)
        .title("Mandelbrot")
        .state(ProgramState::new(
            cfg.default_iterations,
            cfg.max_iterations,
            cfg.click_change_amount,
        ))
        .input(event_handler);

    let mut pixels = calculate_mandelbrot(cfg.default_iterations, width, height);

    canvas.render(move |state, image| {
        // check if we have handled the most recent mouse click
        if state.mandelbrot_state_modified {
            pixels = calculate_mandelbrot(state.mandelbrot_state.iterations, width, height);
            state.mandelbrot_state_modified = false;
        }

        let width = image.width() as usize;
        image.chunks_mut(width).enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, pixel)| {
                *pixel = pixels[x][y];
            });
        });
    });

    Ok(())
}

fn calculate_mandelbrot(iterations: u32, width_u32: u32, height_u32: u32) -> Vec<Vec<Color>> {
    let default_color = Color { r: 0, g: 0, b: 0 };

    let mut pixels = vec![vec![default_color; height_u32 as usize]; width_u32 as usize];

    let width: f32 = width_u32 as f32;
    let height: f32 = height_u32 as f32;

    let w: f32 = 5.0;
    let h: f32 = (w * height) / width;

    let xmin: f32 = -w / 2.0;
    let ymin: f32 = -h / 2.0;

    let xmax: f32 = xmin + w;
    let ymax: f32 = ymin + h;

    // calculate amount we increment x, y for each pixel
    let dx: f32 = (xmax - xmin) / width;
    let dy: f32 = (ymax - ymin) / height;

    let mut y: f32 = ymin;
    for j in 0..height_u32 {
        let mut x: f32 = xmin;
        for i in 0..width_u32 {
            let mut a = x;
            let mut b = y;
            let mut n = 0;
            while n < iterations {
                let a_squared: f32 = a * a;
                let b_squared: f32 = b * b;
                let two_ab: f32 = 2.0 * a * b;

                a = a_squared - b_squared + x;
                b = two_ab + y;
                if a * a + b * b > 16.0 {
                    break;
                }
                n += 1;
            }

            if n != iterations {
                let val = f32::sqrt(n as f32 / iterations as f32);
                let color_val = (val * 255.0) as u8;
                let color = Color {
                    r: color_val,
                    g: color_val,
                    b: color_val,
                };

                pixels[i as usize][j as usize] = color;
            }

            x += dx;
        }
        y += dy;
    }

    pixels
}
