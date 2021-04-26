use crate::GVec3;
use glium::implement_uniform_block;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub ambient: GVec3,
    pub diffuse: GVec3,
    pub specular: GVec3,
    pub shininess: f32,
}
impl Material {
    pub fn new(ambient:  GVec3, diffuse:  GVec3, specular:  GVec3, shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess: shininess * 128.,
        }
    }
}

