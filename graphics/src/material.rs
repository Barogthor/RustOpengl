use crate::GVec3;
use glium::implement_uniform_block;
use crate::uniform::{StructToUniform, UniformStorage};
use glium::uniforms::UniformValue;

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
impl StructToUniform for Material{
    fn as_uniform<'a>(&'a self, struct_name: &str, storage: &mut UniformStorage<'a>) {
        storage.add(&*format!("{}.diffuse", struct_name), UniformValue::Texture2d(&self.diffuse, None));
        storage.add(&*format!("{}.specular", struct_name), UniformValue::Texture2d(&self.specular, None));
        storage.add(&*format!("{}.shininess", struct_name), UniformValue::Float(self.shininess));
    }
}

