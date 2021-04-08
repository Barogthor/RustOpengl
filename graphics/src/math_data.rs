
use math::glm;
use glium::uniforms::{AsUniformValue, UniformValue};
use math::glm::vec3;

#[derive(Debug, Copy, Clone)]
pub struct GVec3 {
    pub data: glm::Vec3
}

impl GVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: vec3(x, y, z)
        }
    }
}

impl AsUniformValue for GVec3{
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Vec3([self.data.x, self.data.y, self.data.z])
    }
}
impl AsUniformValue for &GVec3{
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Vec3([self.data.x, self.data.y, self.data.z])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GMat4 {
    pub data: glm::Mat4,
}

impl GMat4 {
    pub fn new(mat: glm::Mat4) -> Self {
        Self {
            data: mat
        }
    }
}

impl AsUniformValue for GMat4 {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Mat4(self.data.into())
    }
}

impl AsUniformValue for &GMat4 {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Mat4(self.data.into())
    }
}
