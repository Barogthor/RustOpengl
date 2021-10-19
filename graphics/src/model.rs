use obj::{Obj, load_obj};
use crate::{Vertex, Draw};
use std::io::{BufReader, Error};
use std::fs::File;
use wavefront_obj::obj as Wobj;
use crate::glium::{Display, IndexBuffer, VertexBuffer, Frame, Program, DrawParameters, Surface};
use crate::glium::uniforms::Uniforms;

pub struct Mesh{
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>
}

impl Mesh {
    pub fn from(vertexes: Vec<Vertex>, indexes: Vec<u32>, display: &Display) -> Self {
        Self {
            vertex_buffer: VertexBuffer::new(display, &vertexes).unwrap(),
            index_buffer: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indexes).unwrap()
        }
    }
}

pub struct Model {
    meshes: Vec<Mesh>
}
impl Model {
    pub fn new() -> Self {
        Model {
            meshes: vec![]
        }
    }
}
impl Draw for Model{
    fn draw<U>(&self, frame: &mut Frame, program: &Program, uniforms: &U, parameters: &DrawParameters<'_>) where U: Uniforms {
        for mesh in &self.meshes {
            frame.draw(&mesh.vertex_buffer, &mesh.index_buffer, program, uniforms, parameters);
        }
    }
}

pub fn load_model_gltf(path : &str, display: &Display) -> Model{

    let scenes = easy_gltf::load(path).expect(&format!("failed to load file : {}", path));
    let mut model = Model::new();
    // let mat = {
    //     let pbr = &scenes[0].models[0].material().pbr;
    //     let albedo = pbr.base_color_texture.unwrap().
    // };
    for raw_model in &scenes[0].models {
        let mut v = vec![];
        // println!("{:?}", raw_model.material().pbr.);
        for vtx in raw_model.vertices() {
            let pos = vtx.position;
            let norm = vtx.normal;
            let tex = vtx.tex_coords;
            v.push(Vertex::from([pos.x, pos.y, pos.z], [norm.x, norm.y, norm.z], [tex.x, tex.y]))
        }
        let mut ids = vec![];
        for idx in raw_model.indices().unwrap() {
            ids.push((*idx) as u32);
        }
        model.meshes.push(Mesh::from(v, ids, display));
    }
    model
}
