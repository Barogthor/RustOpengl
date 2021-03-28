use winit::event::{MouseButton, VirtualKeyCode as Key};

use crate::input::{Analog2d, Gesture};

pub struct Binding {
    pub exit: Gesture,
    pub movement: Analog2d,
    pub look: Analog2d,
    pub jump: Gesture,
    pub fall: Gesture,
    pub fullscreen: Gesture,
    pub toggle_mouse: Gesture,
    pub swap_color: Gesture,
}

impl Binding {
    pub fn create() -> Self {
        Self {
            exit: Gesture::KeyTrigger(Key::Escape),
            movement: Analog2d::Gestures {
                x_positive: Gesture::AnyOf(vec![
                    Gesture::KeyHold(Key::D),
                    Gesture::KeyHold(Key::Right),
                ]),
                x_negative: Gesture::AnyOf(vec![
                    Gesture::KeyHold(Key::Q),
                    Gesture::KeyHold(Key::Left),
                ]),
                y_positive: Gesture::AnyOf(vec![
                    Gesture::KeyHold(Key::Z),
                    Gesture::KeyHold(Key::Up),
                ]),
                y_negative: Gesture::AnyOf(vec![
                    Gesture::KeyHold(Key::S),
                    Gesture::KeyHold(Key::Down),
                ]),
                step: 0.015,
            },
            look: Analog2d::Mouse { sensitivity: 0.015 },
            jump: Gesture::KeyHold(Key::Space),
            fall: Gesture::KeyHold(Key::C),
            fullscreen: Gesture::KeyTrigger(Key::F),
            toggle_mouse: Gesture::ButtonHold(MouseButton::Right),
            swap_color: Gesture::KeyTrigger(Key::R),
        }
    }
}
