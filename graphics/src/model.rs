use crate::{Vertex, Draw};
use std::io::{BufReader, Error};
use std::fs::File;
use crate::glium::{Display, IndexBuffer, VertexBuffer, Frame, Program, DrawParameters, Surface};
use crate::glium::uniforms::Uniforms;
use russimp::scene::{Scene as aiScene, PostProcess};
use russimp::texture::TextureType;
use russimp::node::Node as aiNode;
use russimp::mesh::Mesh as aiMesh;
use std::rc::Rc;
use std::cell::RefCell;
use russimp::Matrix4x4 as aiMat4;
use math::glm::Mat4;

#[derive(Debug)]
pub struct Mesh {
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

fn convert_matrix4_to_glm(m: aiMat4) -> Mat4 {
    Mat4::from([[m.a1, m.a2, m.a3, m.a4], [m.b1, m.b2, m.b3, m.b4], [m.c1, m.c2, m.c3, m.c4], [m.d1, m.d2, m.d3, m.d4]])
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

    fn process_mesh(&mut self, mesh: &aiMesh, scene: &aiScene, node: &Rc<RefCell<aiNode>>, display: &Display) -> Mesh {
        let mut vertices= vec![];
        let mut indices = vec![];
        let ai_uvs = mesh.texture_coords[0].as_ref();
        if let Some(ai_uvs_first) = ai_uvs {
            for i in 0..mesh.vertices.len() {
                let ai_vertice = mesh.vertices[i];
                let ai_normal = mesh.normals[i];
                let ai_uv = ai_uvs_first[i];
                let t = node.borrow().transformation;
                let old_x = ai_vertice.x;
                let old_y = ai_vertice.y;
                let old_z = ai_vertice.z;
                let new_x = old_x * t.a1 + old_y * t.a2 + old_z * t.a3 + t.a4;
                let new_y = old_x * t.b1 + old_y * t.b2 + old_z * t.b3 + t.b4;
                let new_z = old_x * t.c1 + old_y * t.c2 + old_z * t.c3 + t.c4;
                // println!("({}) {:?}", i, [old_x, old_y, old_z] );
                vertices.push(Vertex::from([new_x, new_y, new_z], [ai_normal.x, ai_normal.y, ai_normal.z], [ai_uv.x, ai_uv.y]));
            }
        }
        else {
            for i in 0..mesh.vertices.len() {
                let ai_vertice = mesh.vertices[i];
                let ai_normal = mesh.normals[i];
                vertices.push(Vertex::from([ai_vertice.x, ai_vertice.y, ai_vertice.z], [ai_normal.x, ai_normal.y, ai_normal.z], [0.0, 0.0]));
            }
        }
        for ai_face in &mesh.faces {
            indices.push(ai_face.0[0]);
            indices.push(ai_face.0[1]);
            indices.push(ai_face.0[2]);
        }
        // println!("{:?}", vertices);
        // println!("[{}] -> {:?}", indices.len(), indices);
        Mesh::from(vertices, indices, display)
    }

    fn process_node(&mut self, node: &Rc<RefCell<aiNode>>, scene: &aiScene ,display: &Display) {
        for v in &scene.meshes[0].vertices {
        }
        for meshId in &node.borrow().meshes {
            let proc_mesh = self.process_mesh(&scene.meshes[*meshId as usize], scene, node, display);
            // println!("{:?}", proc_mesh);
            self.meshes.push(proc_mesh);
        }
        for childNode in &node.borrow().children {
            self.process_node(childNode, scene, display);
        }
    }

    pub fn load_model(path: &str, display: &Display, flip_uv: bool) -> Self{
        let mut model = Self::new();
        let process_steps = if flip_uv {
            vec![PostProcess::CalculateTangentSpace,
                 PostProcess::Triangulate,
                 PostProcess::JoinIdenticalVertices,
                 PostProcess::FlipUVs,
                 PostProcess::SortByPrimitiveType]
        }
        else {
            vec![PostProcess::CalculateTangentSpace,
                 PostProcess::Triangulate,
                 PostProcess::JoinIdenticalVertices,
                 PostProcess::SortByPrimitiveType]
        };
        let scene = aiScene::from_file(path,
                                       process_steps).unwrap();
        if let Some(root) = &scene.root {
            let root_transform = root.borrow().transformation;
            model.process_node(root, &scene, display);
        }
        println!("mesh count: {}", model.meshes.len());
        model
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

pub fn load_model_assimp(path: &str, display: &Display) -> Model {

    let scene = aiScene::from_file(path,
                                 vec![PostProcess::CalculateTangentSpace,
                                      PostProcess::Triangulate,
                                      PostProcess::JoinIdenticalVertices,
                                      PostProcess::GenerateSmoothNormals,
                                      // PostProcess::PreTransformVertices,
                                      // PostProcess::FlipUVs,
                                      PostProcess::SortByPrimitiveType]).unwrap();
    if let Some(meta) = scene.metadata {
        println!("meta datas : {:?}", meta.keys);
    };
    println!("count meshes : {}", scene.meshes.len());
    println!("count materials : {}", scene.materials.len());
    println!("count lights : {}", scene.lights.len());
    println!("count cameras : {}", scene.cameras.len());
    println!("count animations : {}", scene.animations.len());
    let mut meshes = vec![];
    for ai_mesh in scene.meshes {
        // println!("mesh {} :", ai_mesh.name);
        // println!("count vertices : {}", ai_mesh.vertices.len());
        // println!("count uv : {}", ai_mesh.texture_coords[0].as_ref().unwrap().len());
        // println!("count normals : {}", ai_mesh.normals.len());

        let mut vertices= vec![];
        let mut indices = vec![];
        let ai_uvs = ai_mesh.texture_coords[0].as_ref();
        if let Some(ai_uvs_first) = ai_uvs {
            for i in 0..ai_mesh.vertices.len() {
                let ai_vertice = ai_mesh.vertices[i];
                let ai_normal = ai_mesh.normals[i];
                let ai_uv = ai_uvs_first[i];
                vertices.push(Vertex::from([ai_vertice.x, ai_vertice.y, ai_vertice.z], [ai_normal.x, ai_normal.y, ai_normal.z], [ai_uv.x, ai_uv.y]));
            }
        }
        else {
            for i in 0..ai_mesh.vertices.len() {
                let ai_vertice = ai_mesh.vertices[i];
                let ai_normal = ai_mesh.normals[i];
                vertices.push(Vertex::from([ai_vertice.x, ai_vertice.y, ai_vertice.z], [ai_normal.x, ai_normal.y, ai_normal.z], [0.0, 0.0]));
            }
        }
        for ai_face in ai_mesh.faces {
            indices.push(ai_face.0[0]);
            indices.push(ai_face.0[1]);
            indices.push(ai_face.0[2]);
        }

        println!("{:?}", vertices[0]);
        println!("{:?}", vertices[1]);
        println!("{:?}", vertices[2]);
        println!("{:?}", vertices[3]);
        meshes.push(Mesh::from(vertices, indices, display));

    }
    for mat in &scene.materials {
        println!("{:?}", mat);
        break;
        for textype in mat.textures.keys() {
            match textype {
                TextureType::Normals => println!("Normals"),
                TextureType::Ambient => println!("Ambient"),
                TextureType::AmbientOcclusion => println!("AmbientOcclusion"),
                TextureType::BaseColor => println!("Albedo"),
                TextureType::Diffuse => println!("Diffuse"),
                TextureType::Emissive => println!("Emissive"),
                TextureType::EmissionColor => println!("EmissionColor"),
                TextureType::Displacement => println!("Displacement"),
                TextureType::Height => println!("Height"),
                TextureType::Specular => println!("Height"),
                TextureType::Roughness => println!("Roughness"),
                TextureType::Metalness => println!("Metalness"),
                TextureType::Shininess => println!("Shininess"),
                TextureType::Reflection => println!("Reflection"),
                _ => println!("other")
            }
        }
    }
    println!("end import");
    Model {
        meshes
    }
}