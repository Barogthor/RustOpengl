use image::RgbaImage;
use glium::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::ffi::{OsString, OsStr};

fn load_texture_priv(image: RgbaImage, display: &Display) -> Result<glium::texture::Texture2d, ()> {
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
    let file = File::open(file_path).expect(&format!("failed to load file : {}", file_path));
    let buffer = BufReader::new(file);

    let image = image::load(buffer, image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    load_texture_priv(image, display)
}

pub fn load_jpeg_texture(
    file_path: &str,
    display: &Display,
) -> Result<glium::texture::Texture2d, ()> {
    let file = File::open(file_path).expect(&format!("failed to load file : {}", file_path));
    let buffer = BufReader::new(file);

    let image = image::load(buffer, image::ImageFormat::Jpeg)
        .unwrap()
        .to_rgba8();
    load_texture_priv(image, display)
}

pub fn load_tif_texture(
    file_path: &str,
    display: &Display,
) -> Result<glium::texture::Texture2d, ()> {
    let file = File::open(file_path).expect(&format!("failed to load file : {}", file_path));
    let buffer = BufReader::new(file);

    let image = image::load(buffer, image::ImageFormat::Tiff)
        .unwrap()
        .to_rgba8();
    load_texture_priv(image, display)
}