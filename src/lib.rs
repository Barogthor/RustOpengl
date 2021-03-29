use std::fs::File;
use std::io::BufReader;

use glium::{Display, DrawParameters};
use image::RgbaImage;

pub mod input;
pub mod binding;
pub mod geometry;

pub type Vertex = VertexTex;

#[derive(Copy, Clone, Debug)]
pub struct VertexFlat {
    position: [f32; 3],
    // color: [f32;3],
    // tex_coords: [f32; 2],
}

impl VertexFlat {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        VertexFlat {
            position: [x, y, z],
            // tex_coords,
        }
    }
}

glium::implement_vertex!(VertexFlat, position);

#[derive(Copy, Clone, Debug)]
pub struct VertexTex {
    position: [f32; 3],
    // color: [f32;3],
    tex_coords: [f32; 2],
}

impl VertexTex {
    pub fn new(x: f32, y: f32, z: f32, tex_coords: [f32; 2]) -> Self {
        VertexTex {
            position: [x, y, z],
            tex_coords,
        }
    }
}

glium::implement_vertex!(VertexTex, position, tex_coords);


fn load_texture(image: RgbaImage, display: &Display) -> Result<glium::texture::Texture2d, ()> {
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let tex = glium::texture::Texture2d::new(display, image).unwrap();
    Ok(tex)
}

pub fn load_png_texture(
    file_path: &str,
    display: &Display,
) -> Result<glium::texture::Texture2d, ()> {
    let file = File::open(file_path).unwrap();
    let buffer = BufReader::new(file);

    let image = image::load(buffer, image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    load_texture(image, display)
}

pub fn load_jpeg_texture(
    file_path: &str,
    display: &Display,
) -> Result<glium::texture::Texture2d, ()> {
    let file = File::open(file_path).unwrap();
    let buffer = BufReader::new(file);

    let image = image::load(buffer, image::ImageFormat::Jpeg)
        .unwrap()
        .to_rgba8();
    load_texture(image, display)
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