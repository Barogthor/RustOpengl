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

//
fn ext_to_image_type(ext: OsString) -> Option<image::ImageFormat> {
    if ext == "jpg" || ext == "jpeg" {
        Some(image::ImageFormat::Jpeg)
    }
    else if ext == "tiff" || ext == "tif" {
        Some(image::ImageFormat::Tiff)
    }
    else if ext == "png" {
        Some(image::ImageFormat::Png)
    }
    else {
        None
    }
}
pub fn load_texture(file_path: &str, display: &Display) -> Result<glium::texture::Texture2d, ()> {
    let path = Path::new(file_path);
    match path.extension().map(|ext| ext_to_image_type(ext.to_ascii_lowercase())) {
        Some(Some(image::ImageFormat::Jpeg)) => {
            let file = File::open(path).expect(&format!("failed to load file : {}", file_path));
            let buffer = BufReader::new(file);
            let image = image::load(buffer, image::ImageFormat::Jpeg)
                .unwrap()
                .to_rgba8();
            load_texture_priv(image, display)
        }
        Some(Some(image::ImageFormat::Tiff)) => {
            let file = File::open(path).expect(&format!("failed to load file : {}", file_path));
            let buffer = BufReader::new(file);
            let image = image::load(buffer, image::ImageFormat::Tiff)
                .unwrap()
                .to_rgba8();
            load_texture_priv(image, display)
        }
        Some(Some(image::ImageFormat::Png)) => {
            let file = File::open(path).expect(&format!("failed to load file : {}", file_path));
            let buffer = BufReader::new(file);
            let image = image::load(buffer, image::ImageFormat::Png)
                .unwrap()
                .to_rgba8();
            load_texture_priv(image, display)
        }
        _ => Err(())
    }

}

