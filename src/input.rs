use glium::{Display, glutin};
use nalgebra_glm as glm;
use winit::event::{DeviceEvent, VirtualKeyCode};
use winit::event::{Event, MouseButton};

use crate::input::InputState::{Pressed, Released};

const MAX_MOUSE_BUTTONS: usize = 256;
const MAX_KEY_BUTTONS: usize = 512;

pub type Sensitivity = f32;

pub enum Gesture {
    NoGesture,
    KeyHold(VirtualKeyCode),
    KeyTrigger(VirtualKeyCode),
    ButtonHold(MouseButton),
    ButtonTrigger(MouseButton),
    AnyOf(Vec<Gesture>),
    AllOf(Vec<Gesture>),
    QuitTrigger,
}

pub enum Analog2d {
    NoAnalog2d,

    Mouse {
        sensitivity: Sensitivity,
    },

    Gestures {
        x_positive: Gesture,
        x_negative: Gesture,
        y_positive: Gesture,
        y_negative: Gesture,
        step: Sensitivity,
    },

    Sum {
        analogs: Vec<Analog2d>,
    },
}

pub struct Input {
    mouse_buttons: [InputState; MAX_MOUSE_BUTTONS],
    key_buttons: [InputState; MAX_KEY_BUTTONS],
    mouse_wheel_dir: glm::Vec2,
    mouse_rel: glm::Vec2,
    quit_requested_index: u64,
    current_index: u64,
    mouse_grabbed: bool,
    new_mouse_grabbed: bool,
}

impl Input {
    pub fn poll_gesture(&self, gesture: &Gesture) -> bool {
        match *gesture {
            Gesture::QuitTrigger => self.quit_requested_index == self.current_index,
            Gesture::KeyHold(code) => match self.key_buttons[code as usize] {
                InputState::Pressed(_) => true,
                InputState::Released(_) => false,
            },
            Gesture::KeyTrigger(code) => match self.key_buttons[code as usize] {
                InputState::Pressed(index) => self.current_index == index,
                InputState::Released(_) => false,
            },
            Gesture::ButtonHold(button) => match self.mouse_buttons[mouse_button_index(&button)] {
                InputState::Pressed(_) => true,
                InputState::Released(_) => false,
            },
            Gesture::ButtonTrigger(button) => {
                match self.mouse_buttons[mouse_button_index(&button)] {
                    InputState::Pressed(index) => self.current_index == index,
                    InputState::Released(_) => false,
                }
            }
            Gesture::AnyOf(ref subgestures) => subgestures
                .iter()
                .any(|subgesture| self.poll_gesture(subgesture)),
            Gesture::AllOf(ref subgestures) => subgestures
                .iter()
                .all(|subgesture| self.poll_gesture(subgesture)),
            Gesture::NoGesture => false,
        }
    }

    pub fn poll_analog2d(&self, motion: &Analog2d) -> glm::Vec2 {
        match *motion {
            Analog2d::Sum { ref analogs } => analogs
                .iter()
                .map(|analog| self.poll_analog2d(analog))
                .fold(glm::vec2(0., 0.), |v1, v2| v1 + v2),
            Analog2d::Mouse { sensitivity } => self.mouse_rel.clone() * sensitivity,
            Analog2d::Gestures {
                ref x_positive,
                ref x_negative,
                ref y_positive,
                ref y_negative,
                step,
            } => glm::vec2(
                if self.poll_gesture(x_positive) {
                    step
                } else if self.poll_gesture(x_negative) {
                    -step
                } else {
                    0.0
                },
                if self.poll_gesture(y_positive) {
                    step
                } else if self.poll_gesture(y_negative) {
                    -step
                } else {
                    0.0
                },
            ),
            Analog2d::NoAnalog2d => glm::vec2(0., 0.),
        }
    }

    pub fn set_cursor_grabbed(&mut self, grabbed: bool) {
        self.new_mouse_grabbed = grabbed
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        self.mouse_grabbed
    }

    pub fn update(&mut self, window: &Display, event: &Event<()>) {
        if self.new_mouse_grabbed != self.mouse_grabbed {
            self.mouse_grabbed = self.new_mouse_grabbed;
            window
                .gl_window()
                .window()
                .set_cursor_grab(self.mouse_grabbed)
                .ok();
            window
                .gl_window()
                .window()
                .set_cursor_visible(self.mouse_grabbed);
        }
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => match (input.virtual_keycode, input.state) {
                    (Some(key), winit::event::ElementState::Pressed) => {
                        self.key_buttons[key as usize] = Pressed(self.current_index)
                    }
                    (Some(key), winit::event::ElementState::Released) => {
                        self.key_buttons[key as usize] = Released(self.current_index)
                    }
                    (None, _) => {}
                },
                glutin::event::WindowEvent::Focused(flag) => {}
                glutin::event::WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                    ..
                } => match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        self.mouse_wheel_dir += glm::vec2(*x, *y);
                    }
                    _ => {}
                },
                glutin::event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    ..
                } => match state {
                    winit::event::ElementState::Pressed => {
                        self.mouse_buttons[mouse_button_index(button) as usize] =
                            Pressed(self.current_index)
                    }
                    winit::event::ElementState::Released => {
                        self.mouse_buttons[mouse_button_index(button) as usize] =
                            Released(self.current_index)
                    }
                },
                glutin::event::WindowEvent::CursorMoved {
                    device_id,
                    position,
                    ..
                } => {}
                glutin::event::WindowEvent::CloseRequested => {
                    self.quit_requested_index = self.current_index
                }
                _ => {}
            },

            glutin::event::Event::DeviceEvent {
                event: DeviceEvent::Motion { axis, value },
                ..
            } => {
                if *axis < 2 {
                    self.mouse_rel[*axis as usize] += *value as f32;
                }
            }
            _ => {}
        }
    }

    pub fn create() -> Self {
        Self {
            mouse_buttons: [InputState::Released(0); MAX_MOUSE_BUTTONS],
            key_buttons: [InputState::Released(0); MAX_KEY_BUTTONS],
            mouse_wheel_dir: glm::vec2(0.0, 0.0),
            mouse_rel: glm::vec2(0.0, 0.0),
            quit_requested_index: 0,
            current_index: 1,
            mouse_grabbed: false,
            new_mouse_grabbed: false,
        }
    }

    pub fn increment_index(&mut self) {
        self.current_index += 1;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum InputState {
    Pressed(u64),
    Released(u64),
}

fn mouse_button_index(button: &MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
        MouseButton::Other(id) => *id as usize,
    }
}
