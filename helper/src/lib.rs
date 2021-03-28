use rand::Rng;
use std::fs::File;
use std::io::Read;

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

}

impl From<Colors> for (f32, f32,f32) {
    fn from(color: Colors) -> Self {
        match color {
            Colors::RED     => (1., 0., 0.),
            Colors::GREEN   => (0., 1., 0.),
            Colors::BLUE    => (0., 0., 1.),
            Colors::YELLOW  => (1., 1., 0.),
            Colors::MAGENTA => (1., 0., 1.),
            Colors::TEAL    => (0., 1., 1.),
            Colors::GREY    => (0.5, 0.5, 0.5),
            Colors::BLACK   => (0., 0., 0.),
            Colors::WHITE   => (1., 1., 1.),
            Colors::Other(r, g, b, _a) => { (r as f32/ 255., g as f32 / 255., b as f32 / 255.) }
        }
    }
}
impl From<Colors> for (f32, f32, f32, f32) {
    fn from(color: Colors) -> Self {
        match color {
            Colors::RED     => (1., 0., 0., 1.),
            Colors::GREEN   => (0., 1., 0., 1.),
            Colors::BLUE    => (0., 0., 1., 1.),
            Colors::YELLOW  => (1., 1., 0., 1.),
            Colors::MAGENTA => (1., 0., 1., 1.),
            Colors::TEAL    => (0., 1., 1., 1.),
            Colors::GREY    => (0.5, 0.5, 0.5, 1.),
            Colors::BLACK   => (0., 0., 0., 1.),
            Colors::WHITE   => (1., 1., 1., 1.),
            Colors::Other(r, g, b, _a) => { (r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.) }
        }
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


