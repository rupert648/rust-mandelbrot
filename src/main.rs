use pixel_canvas::{Canvas, Color, input::MouseState};

fn main() {
    let width = 1920;
    let height = 1080;
    let max_iterations = 100;
    let pixels = calculate_mandelbrot(max_iterations, width, height);
    println!("===== Finished Calculating... =====");

    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(width as usize, height as usize)
        .title("Mandelbrot")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    // The canvas will render for you at up to 60fps.
    canvas.render(move |_mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = pixels[x][y];
            }
        }
    });
    
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

            let color = if n == max_iterations {
                Color {
                    r: 0,
                    g: 0,
                    b: 0
                }
            } else {
                let val = f32::sqrt(n as f32 / max_iterations as f32);
                let color_val = (val * 255.0) as u8;
                Color {
                    r: color_val,
                    g: color_val,
                    b: color_val
                }
            };

                // draw a 1x1 rect to simulate a pixel
            pixels[i as usize][j as usize] = color;

            x += dx;

        }
        y += dy;
    }

    pixels
}



