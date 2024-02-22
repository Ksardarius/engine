use std::{cell::RefCell, rc::Rc};

use crate::{mesh::{Mesh, Texture, TextureType, Vertex}, shader::Shader};
use russimp::{face::Face, material::Material, mesh, node::Node, scene::{PostProcess, Scene}};

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
    directory: &'static str
}

impl Model {
    pub fn new(path: String) -> Model {
        let mut model = Model {
            meshes: vec![],
            directory: ""
        };

        model.load_model(path);

        model
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in self.meshes.iter() {
            mesh.draw(shader)
        }
    }

    fn load_model(&mut self, path: String) {
        let scene = Scene::from_file(path.as_str(), vec![
            PostProcess::Triangulate,
            PostProcess::FlipUVs
        ]).unwrap();

        // self.directory = &path[..path.rfind('/').unwrap_or(path.len())];

        let root = &scene.root;

        if let Some(root) = root {
            self.process_node(root, &scene);
        }
    }

    fn process_node(&mut self, node: &Rc<Node>, scene: &Scene) {
        for i in node.meshes.iter() {
            let mesh = &scene.meshes[*i as usize];
            self.meshes.push(self.process_mesh(mesh, scene));
        }

        for child in node.children.take().iter() {
            self.process_node(child, scene);
        }
    }

    fn process_mesh(&self, mesh: &mesh::Mesh, scene: &Scene) -> Mesh {
        let mut vertices = vec![];
        let mut indices = vec![];
        let mut textures = vec![];

        for (i, v) in mesh.vertices.iter().enumerate() {
            let vertex = Vertex {
                position: glm::vec3(v.x, v.y, v.z),
                normal: glm::vec3(mesh.normals[i].x, mesh.normals[i].y, mesh.normals[i].z),
                tex_coords: if let Some(t) = &mesh.texture_coords[0] {
                    glm::vec2(t[i].x, t[i].y)
                } else {
                    glm::vec2(0.0, 0.0)
                }
            };

            vertices.push(vertex);
        }

        // process indices
        for Face(i) in mesh.faces.iter() {
            for indice in i {
                indices.push(*indice);
            }
        }

        // process material
        if mesh.material_index >= 0 {
            let materials = &scene.materials;
            println!("Materials???? {:?} {:?}", mesh.material_index, &scene.materials);
            let material = &(materials[mesh.material_index as usize]);
            let diffuse_maps = self.load_material_textures(material, russimp::material::TextureType::Diffuse, "texture_diffuse");
            textures.extend(diffuse_maps);
            let specular_maps = self.load_material_textures(material, russimp::material::TextureType::Specular, "texture_specular");
            textures.extend(specular_maps);

        }

        Mesh::new(vertices, indices, textures)
    }

    fn load_material_textures(&self, mat: &Material, t_type: russimp::material::TextureType, type_name: &str) -> Vec<Texture> {
        let mut textures: Vec<Texture> = vec![];
        println!("Texture???? {:?}", mat.textures);

        if let Some(texture) = &mat.textures.get(&t_type) {
            let filename = String::from(texture.as_ref().borrow().filename.as_str());
            println!("Texture {:?}", filename);
    
            let texture = Texture {
                id: 0, //TextureFromFile
                t_type: match t_type {
                    russimp::material::TextureType::Diffuse => TextureType::TextureDiffuse,
                    russimp::material::TextureType::Specular => TextureType::TextureSpecular,
                    _ => TextureType::TextureDiffuse
                },
                path: filename
            };
    
            textures.push(texture);
        }
        
        textures
    }
}