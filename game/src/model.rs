mod obj_loader;
mod fbx_loader;
mod mesh;


use crate::shader::Shader;

use self::mesh::{Mesh, Texture};

pub enum ModelType {
    OBJ,
    FBX,
}

#[derive(Debug)]
pub struct Model {
    textures_loaded: Vec<Texture>,
    meshes: Vec<Mesh>,
    directory: String,
}

impl Model {
    pub fn load(path: String, model_type: ModelType) -> Model {
        match model_type {
            ModelType::OBJ => obj_loader::load_model(path),
            ModelType::FBX => fbx_loader::load_model(path),
        }
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in self.meshes.iter() {
            mesh.draw(shader)
        }
    }
}
