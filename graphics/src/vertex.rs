
#[derive(Copy, Clone, Debug)]
pub struct VertexFlat {
    position: [f32; 3],
    // color: [f32;3],
    // tex_coords: [f32; 2],
}

impl VertexFlat {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            // tex_coords,
        }
    }
}

glium::implement_vertex!(VertexFlat, position);

#[derive(Copy, Clone, Debug)]
pub struct VertexTex {
    position: [f32; 3],
    // color: [f32;3],
    tex_coords: [f32; 2],
}

impl VertexTex {
    pub fn new(x: f32, y: f32, z: f32, tex_coords: [f32; 2]) -> Self {
        Self {
            position: [x, y, z],
            tex_coords,
        }
    }
}

glium::implement_vertex!(VertexTex, position, tex_coords);


#[derive(Copy, Clone, Debug)]
pub struct VertexNorm {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

impl VertexNorm {
    pub fn new(x: f32, y: f32, z: f32, normal: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position: [x, y, z],
            normal,
            tex_coords,
        }
    }
    pub fn from(position: [f32; 3], normal: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }
}

glium::implement_vertex!(VertexNorm, position, normal, tex_coords);
