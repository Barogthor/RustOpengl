use glium::uniforms::{AsUniformValue, UniformValue};

use helper::Colors;

pub mod input;
pub mod binding;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    // color: [f32;3],
    // tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            position: [x, y, z],
            // tex_coords,
        }
    }
}

glium::implement_vertex!(Vertex, position);
