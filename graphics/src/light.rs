use crate::GVec3;

#[derive(Copy, Clone, Debug)]
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

// implement_uniform_block!(Light, position, ambient, diffuse, specular );

