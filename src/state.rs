pub struct ProgramState {
    pub mandelbrot_state: MandelbrotState,
    /// any time the mandelbrot state is modified
    /// this is set to true -> forces re-calculation
    pub mandelbrot_state_modified: bool,

    /// The x position from the lower-left corner, measured in physical pixels.
    /// This should always correspond to the column of the pixel in the image.
    pub x: i32,
    /// The y position from the lower-left corner, measured in physical pixels.
    /// This should always correspond to the row of the pixel in the image.
    pub y: i32,
    /// The x position from the upper-left corner as reported by the OS,
    /// measured in virtual pixels.
    pub virtual_x: i32,
    /// The y position from the upper-left corner as reported by the OS,
    /// measured in virtual pixels.
    pub virtual_y: i32,

    // camera handling
    pub camera_pos: CameraState,
}

pub struct MandelbrotState {
    pub iterations: u32,
    pub max_iterations: u32,
    pub click_change_amount: u32,
}

///camera position centred onto one pixel
pub struct CameraState {
    pub x: i32,
    pub y: i32,
}

impl ProgramState {
    pub fn new(iterations: u32, max_iterations: u32, click_change_amount: u32) -> Self {
        let camera_pos = CameraState::new();
        let mandelbrot_state =
            MandelbrotState::new(iterations, max_iterations, click_change_amount);

        Self {
            mandelbrot_state,
            mandelbrot_state_modified: false,
            x: 0,
            y: 0,
            virtual_x: 0,
            virtual_y: 0,
            camera_pos,
        }
    }
}

impl MandelbrotState {
    fn new(iterations: u32, max_iterations: u32, click_change_amount: u32) -> Self {
        // default mandelbrot values
        MandelbrotState {
            iterations,
            max_iterations,
            click_change_amount,
        }
    }
}

impl CameraState {
    fn new() -> Self {
        CameraState { x: 0, y: 0 }
    }
}
