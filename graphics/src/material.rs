use crate::GVec3;
use glium::implement_uniform_block;

#[derive(Debug)]
pub struct Material {
    pub diffuse: glium::texture::Texture2d,
    pub specular: glium::texture::Texture2d,
    pub shininess: f32,
}
impl Material {
    pub fn new(diffuse:  glium::texture::Texture2d, specular:  glium::texture::Texture2d, shininess: f32) -> Self {
        Self {
            diffuse,
            specular,
            shininess: shininess * 128.,
        }
    }
}

