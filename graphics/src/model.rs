use crate::{Vertex, Draw, PbrMaterial, load_texture};
use std::io::{BufReader, Error};
use std::fs::File;
use crate::glium::{Display, IndexBuffer, VertexBuffer, Frame, Program, DrawParameters, Surface};
use crate::glium::uniforms::Uniforms;
use russimp::scene::{Scene as aiScene, PostProcess};
use russimp::material::{TextureType as aiTextureType, DataContent};
use russimp::node::Node as aiNode;
use russimp::mesh::Mesh as aiMesh;
use std::rc::Rc;
use std::cell::RefCell;
use russimp::Matrix4x4 as aiMat4;
use math::glm::Mat4;
use std::collections::HashMap;
use russimp::material::Material as aiMaterial;
use std::path::Path;

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

fn convert_matrix4_to_glm(m: &aiMat4) -> Mat4 {
    Mat4::from([[m.a1, m.a2, m.a3, m.a4], [m.b1, m.b2, m.b3, m.b4], [m.c1, m.c2, m.c3, m.c4], [m.d1, m.d2, m.d3, m.d4]])
}

pub struct Model {
    directory: String,
    meshes: Vec<Mesh>,
}
impl Model {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        Model {
            directory: "".to_string(),
            meshes
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct TextureMapKey(aiTextureType, u32);

pub struct ModelLoader<'a> {
    directory: String,
    meshes: Vec<Mesh>,
    mesh_textures_index: HashMap<u32, Vec<String>>,
    // textures: HashMap<aiTextureType, HashMap<String, glium::texture::Texture2d>>,
    textures: HashMap<TextureMapKey, HashMap<String, glium::texture::Texture2d>>,
    current_mat_index: u32,
    display: &'a Display
}

impl<'a> ModelLoader<'a> {
    fn new(display: &'a Display, directory: String) -> Self {
        Self {
            directory,
            meshes: vec![],
            mesh_textures_index: Default::default(),
            textures: Default::default(),
            current_mat_index: 0,
            display
        }
    }

    fn get_path(&self, tex_path: &str) -> String {
        format!("{}/{}", self.directory, tex_path)
    }

    fn process_material(&mut self, material: &aiMaterial, texture_type: aiTextureType, material_index: &u32) {
        let ai_textures_opt = material.textures.get(&texture_type);
        if let Some(ai_texture) = ai_textures_opt {
            // for tex in ai_textures {
                let tex = ai_texture.borrow();
                let full_path = self.get_path(&tex.filename);
                let key = TextureMapKey(texture_type.clone(), *material_index);
                if !self.textures.contains_key(&key) {
                    let loaded_tex = load_texture(&full_path, self.display).unwrap();
                    println!("[{}] [{:?}] path = {:?}",material_index, &key.0, tex.filename);
                    let mut new_hm = HashMap::new();
                    new_hm.insert(full_path.clone(), loaded_tex);
                    self.textures.insert(key, new_hm);
                }
                else if !self.textures.get(&key).unwrap().contains_key(&full_path) {
                    let loaded_tex = load_texture(&full_path, self.display).unwrap();
                    println!("[{}] [{:?}] path = {:?}",material_index ,&texture_type, tex.filename);
                    self.textures.get_mut(&key).unwrap().insert(full_path.clone(), loaded_tex);
                }
            // }
        }
    }

    fn process_mesh(&mut self, mesh: &aiMesh, scene: &aiScene, transform: &Mat4) -> Mesh {
        let mut vertices= vec![];
        let mut indices = vec![];
        let ai_uvs = mesh.texture_coords[0].as_ref();
        if let Some(ai_uvs_first) = ai_uvs {
            for i in 0..mesh.vertices.len() {
                let ai_vertice = mesh.vertices[i];
                let ai_normal = mesh.normals[i];
                let ai_uv = ai_uvs_first[i];
                let t = transform;
                let old_x = ai_vertice.x;
                let old_y = ai_vertice.y;
                let old_z = ai_vertice.z;
                let new_x = old_x * t.m11 + old_y * t.m21 + old_z * t.m31 + t.m41;
                let new_y = old_x * t.m12 + old_y * t.m22 + old_z * t.m32 + t.m42;
                let new_z = old_x * t.m13 + old_y * t.m23 + old_z * t.m33 + t.m43;
                // println!("({}) {:?}", i, [old_x, old_y, old_z] );
                vertices.push(Vertex::from([new_x, new_y, new_z], [ai_normal.x, ai_normal.y, ai_normal.z], [ai_uv.x, ai_uv.y]));
            }
        }
        else {
            for i in 0..mesh.vertices.len() {
                let ai_vertice = mesh.vertices[i];
                let ai_normal = mesh.normals[i];
                let t = transform;
                let old_x = ai_vertice.x;
                let old_y = ai_vertice.y;
                let old_z = ai_vertice.z;
                let new_x = old_x * t.m11 + old_y * t.m21 + old_z * t.m31 + t.m41;
                let new_y = old_x * t.m12 + old_y * t.m22 + old_z * t.m32 + t.m42;
                let new_z = old_x * t.m13 + old_y * t.m23 + old_z * t.m33 + t.m43;
                vertices.push(Vertex::from([new_x, new_y, new_z], [ai_normal.x, ai_normal.y, ai_normal.z], [0.0, 0.0]));
            }
        }
        for ai_face in &mesh.faces {
            indices.push(ai_face.0[0]);
            indices.push(ai_face.0[1]);
            indices.push(ai_face.0[2]);
        }
        if mesh.material_index >= 0 {
            let ai_mat = &scene.materials[mesh.material_index as usize];
            println!("textures types : {:?}", ai_mat.textures.keys());
            // println!("material index : {:?}", mesh.material_index);
            self.current_mat_index = mesh.material_index;
            self.process_material(&ai_mat, aiTextureType::Diffuse, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::Specular, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::Roughness, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::Normals, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::AmbientOcclusion, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::Displacement, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::LightMap, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::Metalness, &mesh.material_index);
            self.process_material(&ai_mat, aiTextureType::Height, &mesh.material_index);
        }
        // println!("{:?}", vertices);
        // println!("[{}] -> {:?}", indices.len(), indices);
        Mesh::from(vertices, indices, self.display)
    }

    fn process_node(&mut self, node: &Rc<aiNode>, scene: &aiScene, parent_transform: &Mat4) {
        let node_b = node;
        let transform = parent_transform * convert_matrix4_to_glm(&node_b.transformation);
        for meshId in &node_b.meshes {
            let proc_mesh = self.process_mesh(&scene.meshes[*meshId as usize], scene, &transform);
            // println!("{:?}", proc_mesh);
            self.meshes.push(proc_mesh);
        }
        for childNode in node_b.children.borrow().iter() {
            self.process_node(childNode, scene, &transform);
        }
    }

    fn process_root_node(&mut self, node: &Rc<aiNode>, scene: &aiScene) {
        let node_b = node;
        let transform = convert_matrix4_to_glm(&node_b.transformation);
        for meshId in &node_b.meshes {
            let proc_mesh = self.process_mesh(&scene.meshes[*meshId as usize], scene, &transform);
            // println!("{:?}", proc_mesh);
            self.meshes.push(proc_mesh);
        }
        for childNode in node.children.borrow().iter() {
            self.process_node(childNode, scene, &transform);
        }
    }

    pub fn load(path: &str, display: &'a Display, flip_uv: bool) -> (Model, PbrMaterial) {
        let start = std::time::Instant::now();
        let directory = Path::new(path)
            .parent()
            .map(|dir| dir.as_os_str())
            .map(|dir| dir.to_os_string().into_string().unwrap())
            .unwrap_or("".into());
        let mut loader = Self::new(display, directory);
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
            loader.process_root_node(root, &scene);
        }
        let ModelLoader { meshes, mut textures, current_mat_index, .. } = loader;
        let material = {
            let mut color_hm =
                textures
                    .remove( &TextureMapKey(aiTextureType::BaseColor, current_mat_index))
                    .or_else(|| textures.remove(&TextureMapKey(aiTextureType::Diffuse, current_mat_index)))
                    .unwrap();
            let key = color_hm.iter().next().unwrap().0.clone();
            let color = color_hm.remove(&key).unwrap();


            let mut normal_hm =
                textures
                    .remove( &TextureMapKey(aiTextureType::Normals, current_mat_index))
                    .or_else(|| textures.remove(&TextureMapKey(aiTextureType::Height, current_mat_index)))
                    .unwrap();
            let key = normal_hm.iter().next().unwrap().0.clone();
            let normal = normal_hm.remove(&key).unwrap();

            let mut reflection_hm =
                textures
                    .remove( &TextureMapKey(aiTextureType::Specular, current_mat_index))
                    .or_else(|| textures.remove(&TextureMapKey(aiTextureType::Roughness, current_mat_index)))
                    .or_else(|| textures.remove(&TextureMapKey(aiTextureType::Shininess, current_mat_index)))
                    .or_else(|| textures.remove(&TextureMapKey(aiTextureType::LightMap, current_mat_index)))
                    .unwrap();
            let key = reflection_hm.iter().next().unwrap().0.clone();
            let reflection = reflection_hm.remove(&key).unwrap();

            PbrMaterial::new(color, reflection, normal)
        };
        let model = Model::new(meshes);
        println!("mesh count: {}", model.meshes.len());
        println!("material count: {}", scene.materials.len());
        // loader.mesh_textures_index.
        let end = std::time::Instant::now();
        println!("model '{}' loaded in {}s", path, end.duration_since(start).as_secs_f64());
        (model, material)
    }
}
