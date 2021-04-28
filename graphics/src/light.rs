use crate::GVec3;
use glium::implement_uniform_block;
use crate::uniform::{StructToUniform, UniformStorage};
use glium::uniforms::AsUniformValue;

#[derive(Debug)]
pub struct Light {
    pub position: GVec3,
    pub ambient: GVec3,
    pub diffuse: GVec3,
    pub specular: GVec3,
}
impl Light {
    pub fn new(position:  GVec3, ambient:  GVec3, diffuse:  GVec3, specular:  GVec3) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            position,

        }
    }
}

impl StructToUniform for Light{
    fn as_uniform<'a>(&'a self, storage: &mut UniformStorage<'a>) {
        storage.add("light.position", self.position.as_uniform_value());
        storage.add("light.ambient", self.ambient.as_uniform_value());
        storage.add("light.diffuse", self.diffuse.as_uniform_value());
        storage.add("light.specular", self.specular.as_uniform_value());
    }
}

