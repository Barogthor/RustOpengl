use glium::uniforms::{AsUniformValue, UniformValue, Uniforms};
use std::collections::HashMap;

// pub enum Uniform {
//     Texture2d(glium::texture::Texture2d),
//     Vec2(f32, f32),
//     Vec3(f32, f32, f32),
//     Mat4([[f32; 4]; 4])
// }
//
// impl AsUniformValue for Uniform {
//     fn as_uniform_value(&self) -> UniformValue<'_> {
//         match self{
//             Uniform::Texture(tex) => UniformValue::Texture2d(tex, None),
//             Uniform::Vec2(x, y) => UniformValue::Vec2([*x, *y]),
//             Uniform::Vec3(x, y, z) => UniformValue::Vec3([*x, *y, *z]),
//             Uniform::Mat4(mat) => UniformValue::Mat4(mat.clone()),
//         }
//     }
// }

pub trait StructToUniform {
    fn as_uniform<'a>(&'a self, struct_name: &str, storage: &mut UniformStorage<'a>);
}

#[derive(Default, Clone)]
pub struct UniformStorage<'a> (HashMap<String, UniformValue<'a>>);

impl<'a> UniformStorage<'a> {
    pub fn add(&mut self, name: &str, value: UniformValue<'a>) {
        self.0.insert(name.to_string(), value);
    }
}

impl Uniforms for UniformStorage<'_> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        self.0.iter().for_each(|(name, uniform)| {
            f(name, *uniform);
        })
    }
}

