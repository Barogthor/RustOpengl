
#[derive(Copy, Clone, Debug)]
pub struct VertexFlat {
    position: [f32; 3],
    // color: [f32;3],
    // tex_coords: [f32; 2],
}

impl VertexFlat {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        VertexFlat {
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
        VertexTex {
            position: [x, y, z],
            tex_coords,
        }
    }
}

glium::implement_vertex!(VertexTex, position, tex_coords);

