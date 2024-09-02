use glad_gl::gl;

use crate::model::mesh::{Mesh, TextureType, Vertex};

use super::{mesh::Texture, Model};

pub fn load_model(path: String) -> Model {
    let mut model = Model {
        textures_loaded: vec![],
        meshes: vec![],
        directory: String::from(&path[..String::from(path.as_str()).rfind('/').unwrap_or(path.len())]),
    };

    let (models, materials) = tobj::load_obj(
        path.as_str(),
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..tobj::LoadOptions::default()
        },
    )
    .expect("Failed to OBJ load file");

    // Note: If you don't mind missing the materials, you can generate a default.
    let materials = materials.expect("Failed to load MTL file");

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let generated_mesh = process_mesh(&mut model, mesh, &materials);
        model.meshes.push(generated_mesh);
    }

    model
}

fn process_mesh(model: &mut Model, mesh: &tobj::Mesh, materials: &Vec<tobj::Material>) -> Mesh {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut textures = vec![];

    assert!(mesh.positions.len() % 3 == 0);
    for vtx in 0..mesh.positions.len() / 3 {
        let vertex = Vertex {
            position: glm::vec3(mesh.positions[3 * vtx], mesh.positions[3 * vtx + 1], mesh.positions[3 * vtx + 2]),
            normal: glm::vec3(mesh.normals[3 * vtx], mesh.normals[3 * vtx + 1], mesh.normals[3 * vtx + 2]),
            tex_coords: if mesh.texcoords.len() > 0 {
                glm::vec2(mesh.texcoords[2 * vtx], mesh.texcoords[2 * vtx + 1])
            } else {
                glm::vec2(0.0, 0.0)
            },
        };

        vertices.push(vertex);
    }

    // process indices
    indices.extend(mesh.indices.iter());

    // process material
    if let Some(material_id) = mesh.material_id {
        let material = &materials[material_id];
        let (diffuse_maps, diffuse_loaded) = load_material_textures(model, material, TextureType::TextureDiffuse);
        textures.extend(diffuse_maps);
        let (ao_maps, ao_loaded) = load_material_textures(model, material, TextureType::TextureAmbientOclusion);
        textures.extend(ao_maps);
        let (metallic_maps, metallic_loaded) = load_material_textures(model, material, TextureType::TextureMetallic);
        textures.extend(metallic_maps);
        // let (specular_maps, specular_loaded) = self.load_material_textures(material, TextureType::TextureSpecular);
        // textures.extend(specular_maps);
        let (normal_maps, normal_loaded) = load_material_textures(model, material, TextureType::TextureNormal);
        textures.extend(normal_maps);
        let (roughness_maps, roughness_loaded) = load_material_textures(model, material, TextureType::TextureRoughness);
        textures.extend(roughness_maps);

        model.textures_loaded.extend(diffuse_loaded);
        model.textures_loaded.extend(ao_loaded);
        // self.textures_loaded.extend(specular_loaded);
        model.textures_loaded.extend(normal_loaded);
        model.textures_loaded.extend(metallic_loaded);
        model.textures_loaded.extend(roughness_loaded);
    }
    // if mesh.material_index >= 0 {
    //     let materials = &scene.materials;
    //     println!(
    //         "Materials???? {:?} {:?}",
    //         mesh.material_index, &scene.materials
    //     );
    //     let material = &(materials[mesh.material_index as usize]);
    //     let diffuse_maps = self.load_material_textures(
    //         material,
    //         russimp::material::TextureType::Diffuse,
    //         "texture_diffuse",
    //     );
    //     textures.extend(diffuse_maps);
    //     let specular_maps = self.load_material_textures(
    //         material,
    //         russimp::material::TextureType::Specular,
    //         "texture_specular",
    //     );
    //     textures.extend(specular_maps);
    // }

    Mesh::new(vertices, indices, textures)
}

fn load_material_textures(
    model: &mut Model,
    mat: &tobj::Material,
    t_type: TextureType,
    // type_name: &str,
) -> (Vec<Texture>, Vec<Texture>) {
    let mut textures: Vec<Texture> = vec![];
    let mut textures_loaded = vec![];
    //println!("Texture???? {:?}", mat.textures);

    let directory = &model.directory;

    // let mut a: Option<&String>;

    let texture = if let Some(tex_path) = match t_type {
        TextureType::TextureDiffuse => mat.diffuse_texture.as_ref(),
        // TextureType::TextureSpecular => mat.specular_texture.as_ref(),
        TextureType::TextureNormal => mat.normal_texture.as_ref(),
        TextureType::TextureMetallic => mat.unknown_param.get("map_refl"),
        TextureType::TextureRoughness => mat.unknown_param.get("map_Pr"),
        TextureType::TextureAmbientOclusion => mat.unknown_param.get("map_Po"),
    } {
        let filename = format!("{directory}/{tex_path}");

        if let Some(added_texture) = model.textures_loaded.iter().find(|&tex| tex.path == filename) {
            Some(added_texture.clone())
        } else {
            let texture = Texture {
                id: texture_from_file(filename.as_str()).unwrap(),
                t_type: t_type,
                path: filename,
            };
            textures_loaded.push(texture.clone());

            Some(texture)
        }
    } else {
        None
    };

    if let Some(tex) = texture {
        textures.push(tex);
    }

    (textures, textures_loaded)
}

fn texture_from_file(filename: &str) -> Result<gl::GLuint, Box<dyn std::error::Error>> {  
    let mut texture_id: gl::GLuint = 0;

    println!("Texture???? {:?}", filename);

    unsafe {
        gl::GenTextures(1, &mut texture_id as *mut gl::GLuint);

        // let img = image::open(path)?
        //     //.unwrap()
        //     .flipv()
        //     .into_rgba8();

        let img = image::open(filename)?;
        let channel_count = img.color().channel_count();

        let format = match channel_count {
            1 => gl::RED,
            3 => gl::RGB,
            4 => gl::RGBA,
            _ => gl::RGBA
        };

        
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::LINEAR as i32,
        );

        Ok(texture_id)
    }
}