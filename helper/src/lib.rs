use rand::Rng;
use std::fs::File;
use std::io::Read;
pub use nalgebra_glm as glm;

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


pub fn get_perspective(width: u32, height: u32) -> glm::Mat4 {
    glm::perspective(
        width as f32 / height as f32,
        std::f64::consts::FRAC_PI_4 as f32,
        0.1,
        100.0,
    )
}

pub fn get_camera() -> glm::Mat4 {
    glm::look_at(
        // &glm::vec3(10.0, 4.0, -1.0),
        &glm::vec3(2.0, 0.0, -2.0),
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 1.0, 0.0f32),
    )
}