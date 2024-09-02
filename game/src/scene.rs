use std::fs;
use protobuf::text_format;

pub mod config {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

impl config::scene::Scene {
    pub fn load_scene(path: &str) -> config::scene::Scene {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        let scene = text_format::parse_from_str::<config::scene::Scene>(&contents).unwrap();
        scene
    }
}
