mod colors;
mod vertex;
mod texture;
mod material;
mod light;
mod math_data;
pub mod uniform;
pub mod model;

pub use colors::Colors;
pub use glium;
pub use vertex::*;
pub use texture::*;
pub use material::*;
pub use light::*;
pub use math_data::*;

use glium::{DrawParameters, Program};
use crate::vertex::VertexNorm;
use std::fs::File;
use std::io::Read;
use crate::glium::Frame;
use crate::glium::uniforms::Uniforms;

pub type Vertex = VertexNorm;

pub fn load_glsl(path: &str) -> String {
    let mut nice_shader = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut nice_shader)
        .unwrap();
    nice_shader
}

pub fn draw_params() -> DrawParameters<'static> {
    use glium::{Depth, DepthTest, BackfaceCullingMode};
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            ..Depth::default()
        },
        backface_culling: BackfaceCullingMode::CullClockwise,
        ..DrawParameters::default()
    }
}

pub trait Draw {
    fn draw<U>(&self, frame: &mut Frame, program: &Program, uniforms: &U, parameters: &DrawParameters<'_>) where U : Uniforms;
}

