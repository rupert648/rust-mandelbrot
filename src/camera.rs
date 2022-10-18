use pixel_canvas::input::glutin::dpi::PhysicalPosition;

///camera position centred onto one pixel
pub struct CameraState {
    pub x: i32,
    pub y: i32,
    pub mouse_start_pos: PhysicalPosition<i32>,
}

impl CameraState {
    pub fn new() -> Self {
        CameraState {
            x: 0,
            y: 0,
            mouse_start_pos: PhysicalPosition { x: 0, y: 0 },
        }
    }
}

pub fn move_camera(mouse_position: PhysicalPosition<f64>) {
    todo!()
}
