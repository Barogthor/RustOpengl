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
    fn as_uniform<'a>(&'a self, struct_name: &str, storage: &mut UniformStorage<'a>) {
        storage.add(&*format!("{}.position", struct_name), self.position.as_uniform_value());
        storage.add(&*format!("{}.ambient", struct_name), self.ambient.as_uniform_value());
        storage.add(&*format!("{}.diffuse", struct_name), self.diffuse.as_uniform_value());
        storage.add(&*format!("{}.specular", struct_name), self.specular.as_uniform_value());
        storage.add(&*format!("{}.constant", struct_name), self.constant.as_uniform_value());
        storage.add(&*format!("{}.linear", struct_name), self.linear.as_uniform_value());
        storage.add(&*format!("{}.quadratic", struct_name), self.quadratic.as_uniform_value());
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
    fn as_uniform<'a>(&'a self, struct_name: &str, storage: &mut UniformStorage<'a>) {
        storage.add(&*format!("{}.direction", struct_name), self.direction.as_uniform_value());
        storage.add(&*format!("{}.ambient", struct_name), self.ambient.as_uniform_value());
        storage.add(&*format!("{}.diffuse", struct_name), self.diffuse.as_uniform_value());
        storage.add(&*format!("{}.specular", struct_name), self.specular.as_uniform_value());
    }
}

#[derive(Debug)]
pub struct SpotLight {
    pub position: GVec3,
    pub direction: GVec3,
    pub ambient: GVec3,
    pub diffuse: GVec3,
    pub specular: GVec3,
    pub cut_off: f32, // pass the cosine of a radians angle to avoid heavy calculation in shader
    pub outer_cut_off: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}
impl SpotLight {
    pub fn new(position:  GVec3, direction:  GVec3, ambient:  GVec3, diffuse:  GVec3, specular:  GVec3,
               constant: f32, linear: f32, quadratic: f32, cut_off: f32, outer_cut_off: f32) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            direction,
            cut_off,
            outer_cut_off,
            constant,
            linear,
            quadratic
        }
    }
}

impl StructToUniform for SpotLight{
    fn as_uniform<'a>(&'a self, struct_name: &str, storage: &mut UniformStorage<'a>) {
        storage.add(&*format!("{}.position", struct_name), self.position.as_uniform_value());
        storage.add(&*format!("{}.direction", struct_name), self.direction.as_uniform_value());
        storage.add(&*format!("{}.ambient", struct_name), self.ambient.as_uniform_value());
        storage.add(&*format!("{}.diffuse", struct_name), self.diffuse.as_uniform_value());
        storage.add(&*format!("{}.specular", struct_name), self.specular.as_uniform_value());
        storage.add(&*format!("{}.cutOff", struct_name), self.cut_off.as_uniform_value());
        storage.add(&*format!("{}.outerCutOff", struct_name), self.outer_cut_off.as_uniform_value());
        storage.add(&*format!("{}.constant", struct_name), self.constant.as_uniform_value());
        storage.add(&*format!("{}.linear", struct_name), self.linear.as_uniform_value());
        storage.add(&*format!("{}.quadratic", struct_name), self.quadratic.as_uniform_value());
    }
}