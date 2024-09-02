use flecs_ecs::core::{EntityView, World};
use minidom::Element;

use super::{cube_physics_component::CubePhysicsComponent, teapot_render_component::TeapotRenderComponent, ACTOR_NS};

pub struct ActorPrefab;

impl ActorPrefab {
    pub fn new<'a>(root: &Element, world: &'a World) -> EntityView<'a> {
        let name = root.attr("name").unwrap_or_default();
        let prefab = world.prefab_named(name);

        for child in root.children() {
            if child.is("CubePhysicsComponent", ACTOR_NS) {
                let component: CubePhysicsComponent = child.into();
                prefab.set(component);
            }
    
            if child.is("TeapotRenderComponent", ACTOR_NS) {
                let component: TeapotRenderComponent = child.into();
                prefab.set(component);
            }
        }

        prefab
    }
}