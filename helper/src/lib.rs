use rand::Rng;
use std::fs::File;
use std::io::Read;
pub use nalgebra_glm as glm;
use crate::glm::Mat4;

pub type RawMat4 = [[f32; 4]; 4];

#[derive(Clone, PartialOrd, PartialEq, Debug, Copy)]
pub enum Colors {
    RED,
    BLUE,
    GREEN,
    MAGENTA,
    YELLOW,
    TEAL,
    WHITE,
    GREY,
    BLACK,
    Other(u8, u8, u8, u8)
}

impl Colors {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let n = rng.gen::<u32>();
        match n % 9 {
            0 => Colors::RED,
            1 => Colors::BLUE,
            2 => Colors::GREEN,
            3 => Colors::MAGENTA,
            4 => Colors::YELLOW,
            5 => Colors::TEAL,
            6 => Colors::WHITE,
            7 => Colors::GREY,
            8 => Colors::BLACK,
            _ => panic!("Unexpected random for color conversion"),
        }
    }

    fn to_tuple(self) -> (u8, u8, u8, u8) {
        match self {
            Colors::RED => (255, 0, 0, 255),
            Colors::GREEN => (0, 255, 0, 255),
            Colors::BLUE => (0, 0, 255, 255),
            Colors::YELLOW => (255, 255, 0, 255),
            Colors::MAGENTA => (255, 0, 255, 255),
            Colors::TEAL => (0, 255, 255, 255),
            Colors::GREY => (128, 128, 128, 255),
            Colors::BLACK => (0, 0, 0, 255),
            Colors::WHITE => (255, 255, 255, 255),
            Colors::Other(r, g, b, a) => { (r, g, b, a) }
        }
    }


}

impl From<Colors> for (f32, f32,f32) {
    fn from(color: Colors) -> Self {
        let color = color.to_tuple();
        (color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255.)
    }
}
impl From<Colors> for (f32, f32, f32, f32) {
    fn from(color: Colors) -> Self {
        let color = color.to_tuple();
        (color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255., color.3 as f32 / 255.)
    }
}
impl From<Colors> for [f32; 3] {
    fn from(color: Colors) -> Self {
        let color = color.to_tuple();
        [color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255.]
    }
}
impl From<Colors> for [f32; 4] {
    fn from(color: Colors) -> Self {
        let color = color.to_tuple();
        [color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255., color.3 as f32 / 255.]
    }
}


pub fn load_glsl(path: &str) -> String {
    let mut nice_shader = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut nice_shader)
        .unwrap();
    nice_shader
}


pub struct Perspective{
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

impl Perspective {
    pub fn get(&self) -> Mat4 {
        glm::perspective(self.aspect, self.fov, self.near, self.far)
    }
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            aspect: 1024. / 768.,
            fov: std::f64::consts::FRAC_PI_4 as f32,
            near: 0.1,
            far: 100.0
        }
    }
}

pub struct CameraSystem {
    pub pos: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3
}

impl CameraSystem {
    pub fn view(&self) -> glm::Mat4 {
        glm::look_at(&self.pos, &(&self.pos + &self.front), &self.up)
    }
}

impl Default for CameraSystem {
    fn default() -> Self {
        Self {
            pos: glm::vec3(0.0, 0.0, -7.0),
            front: glm::vec3(0.0, 0.0, 1.0),
            up: glm::vec3(0.0, 1.0, 0.0f32),
        }
    }
}
impl From<&CameraSystem> for Mat4{
    fn from(cam: &CameraSystem) -> Self {
        cam.view()
    }
}


pub fn get_camera() -> glm::Mat4 {
    glm::look_at(
        // &glm::vec3(10.0, 4.0, -1.0),
        &glm::vec3(0.0, 0.0, 3.0),
        &glm::vec3(0.0, 0.0, 2.0),
        &glm::vec3(0.0, 1.0, 0.0f32),
    )
}

pub struct Transform {
    transform: glm::Mat4,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            transform: glm::identity(),
        }
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32){
        self.transform = glm::scale(&self.transform, &glm::vec3(x, y, z));
    }

    pub fn move_to(&mut self, _x: f32, _y: f32, _z: f32) {
        unimplemented!()
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32){
        self.transform = glm::translate(&self.transform, &glm::vec3(x, y, z));
    }

    pub fn rotate(&mut self, angle: f32, axis: &glm::Vec3) {
        self.transform = glm::rotate(&self.transform, angle, axis);
    }

    pub fn get(&self) -> &glm::Mat4 {
        &self.transform
    }
    pub fn get_raw(&self) -> RawMat4 {
        self.transform.clone().into()
    }
}

impl From<&Transform> for RawMat4 {
    fn from(v: &Transform) -> Self {
        v.transform.clone().into()
    }
}

pub struct TransformBuilder(Transform);

impl TransformBuilder {
    pub fn new()-> Self {
        Self(Transform::new())
    }

    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.0.scale(x, y, z);
        self
    }
    pub fn rotate(mut self, angle: f32, axis: &glm::Vec3) -> Self {
        self.0.rotate(angle, axis);
        self
    }
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.0.translate(x, y , z);
        self
    }
    pub fn build(self) -> Transform {
        self.0
    }
}