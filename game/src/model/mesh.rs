use std::{mem, os::raw::c_void};

use glad_gl::gl;

use crate::shader::Shader;

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

#[derive(Debug, Clone)]
pub enum TextureType {
    TextureDiffuse,
    // TextureSpecular,
    TextureNormal,
    TextureMetallic,
    TextureRoughness,
    TextureAmbientOclusion
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub t_type: TextureType,
    pub path: String
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,

    vao: u32, 
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,

            vao: 0,
            vbo: 0,
            ebo: 0
        };

        mesh.setup_mesh();

        mesh
    }

    pub fn draw(&self, shader: &Shader) {
        let mut diffuse_nr = 1;
        let mut ao_nr = 1;
        let mut metallic_nr = 1;
        let mut normal_nr = 1;
        let mut roughness_nr = 1;
        

        unsafe {
            for (i, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32); // activate proper texture unit before binding
                // retrieve texture number (the N in diffuse_textureN)
                let (name, number) = match texture.t_type {
                    TextureType::TextureDiffuse => {
                        let res = diffuse_nr;
                        diffuse_nr += 1;
                        ("texture_diffuse", res.to_string())
                    },
                    TextureType::TextureAmbientOclusion => {
                        let res = ao_nr;
                        ao_nr += 1;
                        ("texture_ao", res.to_string())
                    },
                    TextureType::TextureMetallic => {
                        let res = metallic_nr;
                        metallic_nr += 1;
                        ("texture_metallic", res.to_string())
                    },
                    TextureType::TextureNormal => {
                        let res = normal_nr;
                        normal_nr += 1;
                        ("texture_normal", res.to_string())
                    },
                    TextureType::TextureRoughness => {
                        let res = roughness_nr;
                        roughness_nr += 1;
                        ("texture_roughness", res.to_string())
                    }
                };

                shader.set_int(format!("material.{name}{number}").as_str(), i as i32);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            gl::ActiveTexture(gl::TEXTURE0);

            // draw mesh
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null::<c_void>());
            gl::BindVertexArray(0);
        }
        
    }

    fn setup_mesh(&mut self) {
        unsafe {
            gl::GenVertexArrays(1,  &mut self.vao as *mut gl::GLuint);
            gl::GenBuffers(1, &mut  self.vbo as *mut gl::GLuint);
            gl::GenBuffers(1, &mut  self.ebo as *mut gl::GLuint);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            println!("Size: {:?} Offset: {:?} + {:?}, ", mem::size_of::<Vertex>(), mem::offset_of!(Vertex, normal), mem::offset_of!(Vertex, tex_coords));

            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                std::ptr::null::<c_void>(),
            );

            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                std::ptr::null::<c_void>().offset(mem::offset_of!(Vertex, normal) as isize),
            );

            // vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                std::ptr::null::<c_void>().offset(mem::offset_of!(Vertex, tex_coords) as isize),
            );

            gl::BindVertexArray(0);
        }
        
    }
}