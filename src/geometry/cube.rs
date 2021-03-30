use crate::Vertex;

pub fn cube_indexes() -> [u16; 36] {
    [
        0, 2, 1, // front
        1, 2, 3, 4, 5, 6, // back
        5, 7, 6, 6, 7, 8, //top
        7, 9, 8, 1, 3, 4, //bottom
        3, 5, 4, 1, 11, 10, // left
        1, 4, 11, 3, 12, 5, //right
        5, 12, 13u16,
    ]
}

pub fn cube_vertexes() -> [Vertex; 14] {
    [
        Vertex::new(0.0, 1.0, 0.0, [0.00, 0.66]),
        Vertex::new(0.0, 0.0, 0.0, [0.25, 0.66]),
        Vertex::new(1.0, 1.0, 0.0, [0.00, 0.33]),
        Vertex::new(1.0, 0.0, 0.0, [0.25, 0.33]),
        Vertex::new(0.0, 0.0, 1.0, [0.50, 0.66]),
        Vertex::new(1.0, 0.0, 1.0, [0.50, 0.33]),
        Vertex::new(0.0, 1.0, 1.0, [0.75, 0.66]),
        Vertex::new(1.0, 1.0, 1.0, [0.75, 0.33]),
        Vertex::new(0.0, 1.0, 0.0, [1.00, 0.66]),
        Vertex::new(1.0, 1.0, 0.0, [1.00, 0.33]),
        Vertex::new(0.0, 1.0, 0.0, [0.25, 1.00]),
        Vertex::new(0.0, 1.0, 1.0, [0.50, 1.00]),
        Vertex::new(1.0, 1.0, 0.0, [0.25, 0.00]),
        Vertex::new(1.0, 1.0, 1.0, [0.50, 0.00]),
    ]
}

// pub const UV_MAP: [f32; 28] = [
//     0., 0.66, 0.25, 0.66, 0., 0.33, 0.25, 0.33, 0.5, 0.66, 0.5, 0.33, 0.75, 0.66, 0.75, 0.33, 1.,
//     0.66, 1., 0.33, 0.25, 1., 0.5, 1., 0.25, 0., 0.5, 0.,
// ];