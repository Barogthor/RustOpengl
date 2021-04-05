use glium::implement_uniform_block;

#[derive(Copy, Clone)]
pub struct Material {
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32),
    pub shininess: f32,
}
impl Material {
    pub fn new(ambient: math::glm::Vec3, diffuse: math::glm::Vec3, specular: math::glm::Vec3, shininess: f32) -> Self {
        Self {
            ambient: (ambient.x, ambient.y, ambient.z),
            diffuse: (diffuse.x, diffuse.y, diffuse.z),
            specular: (specular.x, specular.y, specular.z),
            shininess: shininess * 128.
        }
    }
}

implement_uniform_block!(Material, ambient, diffuse, specular, shininess);

