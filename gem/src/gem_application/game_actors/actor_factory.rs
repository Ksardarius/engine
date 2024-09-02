use std::{fs::File, io::BufReader};

use flecs_ecs::core::{EntityView, World};
use minidom::Element;

use super::{actor::{Actor, ActorId}, actor_prefab::ActorPrefab};

pub struct ActorFactory {
    last_actor_id: ActorId
}

impl ActorFactory {
    pub fn create_actor(&mut self, path: &str) -> Actor {
        let f = File::open(path).unwrap();
        let f = BufReader::new(f);

        let root = Element::from_reader(f).unwrap();

        let mut actor: Actor = (&root).into();
        actor.set_id(self.get_next_actor_id());
        actor
    }

    pub fn create_prefab<'a>(&mut self, path: &str, world: &'a World) -> EntityView<'a> {
        let f = File::open(path).unwrap();
        let f = BufReader::new(f);

        let root = Element::from_reader(f).unwrap();

        ActorPrefab::new(&root, world)
    }

    fn get_next_actor_id(&mut self) -> ActorId {
        self.last_actor_id = self.last_actor_id + 1;
        self.last_actor_id
    }

    pub fn new(last_actor_id: ActorId) -> Self {
        Self {
            last_actor_id
        }
    }
}