use glium::uniforms::{AsUniformValue, UniformValue};

pub enum Uniform {
    Texture(glium::texture::Texture2d),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Mat4([[f32; 4]; 4])
}

impl AsUniformValue for Uniform {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        match self{
            Uniform::Texture(tex) => UniformValue::Texture2d(tex, None),
            Uniform::Vec2(x, y) => UniformValue::Vec2([*x, *y]),
            Uniform::Vec3(x, y, z) => UniformValue::Vec3([*x, *y, *z]),
            Uniform::Mat4(mat) => UniformValue::Mat4(mat.clone()),
        }
    }
}