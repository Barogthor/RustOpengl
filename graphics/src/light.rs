use crate::GVec3;
use glium::implement_uniform_block;
use crate::uniform::{StructToUniform, UniformStorage};
use glium::uniforms::AsUniformValue;

#[derive(Debug)]
pub struct PointLight {
    pub position: GVec3,
    pub ambient: GVec3,
    pub diffuse: GVec3,
    pub specular: GVec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}
impl PointLight {
    pub fn new(position:  GVec3, ambient:  GVec3, diffuse:  GVec3, specular:  GVec3,
               constant: f32, linear: f32, quadratic: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            position,

            quadratic
        }
    }
}

impl StructToUniform for PointLight {
    fn as_uniform<'a>(&'a self, storage: &mut UniformStorage<'a>) {
        storage.add("light.position", self.position.as_uniform_value());
        storage.add("light.ambient", self.ambient.as_uniform_value());
        storage.add("light.diffuse", self.diffuse.as_uniform_value());
        storage.add("light.specular", self.specular.as_uniform_value());
        storage.add("light.constant", self.constant.as_uniform_value());
        storage.add("light.linear", self.linear.as_uniform_value());
        storage.add("light.quadratic", self.quadratic.as_uniform_value());
    }
}


#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: GVec3,
    pub ambient: GVec3,
    pub diffuse: GVec3,
    pub specular: GVec3,
}
impl DirectionalLight {
    pub fn new(position:  GVec3, ambient:  GVec3, diffuse:  GVec3, specular:  GVec3) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            direction: position,

        }
    }
}

impl StructToUniform for DirectionalLight{
    fn as_uniform<'a>(&'a self, storage: &mut UniformStorage<'a>) {
        storage.add("light.direction", self.direction.as_uniform_value());
        storage.add("light.ambient", self.ambient.as_uniform_value());
        storage.add("light.diffuse", self.diffuse.as_uniform_value());
        storage.add("light.specular", self.specular.as_uniform_value());
    }
}

