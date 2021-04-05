use glium::implement_uniform_block;

#[derive(Copy, Clone)]
pub struct Light {
    pub position: (f32, f32, f32),
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32),
}
impl Light {
    pub fn new(position: math::glm::Vec3, ambient: math::glm::Vec3, diffuse: math::glm::Vec3, specular: math::glm::Vec3) -> Self {
        Self {
            ambient: (ambient.x, ambient.y, ambient.z),
            diffuse: (diffuse.x, diffuse.y, diffuse.z),
            specular: (specular.x, specular.y, specular.z),
            position: (position.x, position.y, position.z),

        }
    }
}

implement_uniform_block!(Light, position, ambient, diffuse, specular );

