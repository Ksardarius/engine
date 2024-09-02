use super::Model;
use fbxcel_dom::any::AnyDocument;
use russimp::scene::{PostProcess, Scene};

pub fn load_model(path: String) -> Model {
    let mut model = Model {
        textures_loaded: vec![],
        meshes: vec![],
        directory: String::from(
            &path[..String::from(path.as_str()).rfind('/').unwrap_or(path.len())],
        ),
    };

    // let file = std::fs::File::open(path).expect("Failed to open file");
    // // You can also use raw `file`, but do buffering for better efficiency.
    // let reader = std::io::BufReader::new(file);

    // // Use `from_seekable_reader` for readers implementing `std::io::Seek`.
    // // To use readers without `std::io::Seek` implementation, use `from_reader`
    // // instead.
    // match AnyDocument::from_seekable_reader(reader).expect("Failed to load document") {
    //     AnyDocument::V7400(fbx_ver, doc) => {
    //         println!("Loaded FBX DOM successfully: FBX version = {:?}", fbx_ver);
    //         for scene in doc.scenes() {
    //             println!("Scene object: object_id={:?}", scene.object_id());
    //             let root_id = scene
    //                 .root_object_id()
    //                 .expect("Failed to get root object ID");
    //             println!("\tRoot object ID: {:?}", root_id);
    //         }
    //     }
    //     // `AnyDocument` is nonexhaustive.
    //     // You should handle unknown document versions case.
    //     _ => panic!("Got FBX document of unsupported version"),
    // }

    let scene = Scene::from_file(
        path.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    model
}
