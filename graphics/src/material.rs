use crate::GVec3;

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

// implement_uniform_block!(Material, ambient, diffuse, specular, shininess);



// impl Uniforms for &Material{
//     fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
//         f("material.ambient", UniformValue::Vec3([self.ambient.x, self.ambient.y, self.ambient.z]));
//         f("material.diffuse", UniformValue::Vec3([self.diffuse.x, self.diffuse.y, self.diffuse.z]));
//         f("material.specular", UniformValue::Vec3([self.specular.x, self.specular.y, self.specular.z]));
//         f("material.shininess", UniformValue::Float(self.shininess));
//     }
// }

// implement_buffer_content!(Material);