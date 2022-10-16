use pixel_canvas::input::glutin::event::{ElementState, MouseButton};

pub struct ProgramState {
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


    /// the current state of the mouse
    /// `Either Pressed` or `Released`
    pub mouse_state: ElementState,
    /// the last mouse button pressed
    pub mouse_button: MouseButton,
    ///
    /// 
    pub mouse_click_handled: bool,
}


impl ProgramState {
    pub fn new() -> Self {
        Self { 
            x: 0,
            y: 0,
            virtual_x: 0,
            virtual_y: 0,
            mouse_state: ElementState::Released,
            mouse_button: MouseButton::Left,
            mouse_click_handled: true
        }
    }
}