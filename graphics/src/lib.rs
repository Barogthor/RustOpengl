mod colors;
mod vertex;
mod texture;

pub use colors::Colors;
pub use glium;
pub use vertex::*;
pub use texture::*;

use glium::DrawParameters;
use crate::vertex::VertexNorm;
use std::fs::File;
use std::io::Read;

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


