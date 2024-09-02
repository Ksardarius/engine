use std::{collections::HashMap, rc::Rc};

use minidom::Element;

use super::{actor_component::{ActorComponent, ComponentId}, cube_physics_component::CubePhysicsComponent, teapot_render_component::TeapotRenderComponent, ACTOR_NS};

type ActorComponents = HashMap<ComponentId, Box<dyn ActorComponent>>;
pub type ActorId = i64;

#[derive(Debug)]
pub struct Actor {
    id: ActorId,
    pub components: ActorComponents
}

impl Actor {
    pub fn get_id(self) -> ActorId {
        self.id
    }

    pub fn set_id(&mut self, id: ActorId) {
        self.id = id;
    }

    pub fn get_component<T: ActorComponent + 'static>(&self, id: ComponentId) -> Option<&T> {
        let c = self.components.get(&id)?;
        let c = c.as_any().downcast_ref::<T>()?;
        Some(c)
    }

    pub fn get_component_mut<T: ActorComponent + 'static>(&mut self, id: ComponentId) -> Option<&mut T> {
        let c = self.components.get_mut(&id)?;
        let c = c.as_any_mut().downcast_mut::<T>()?;
        Some(c)
    }

    pub fn update(&mut self, delta_ms: i64) {
        self.components.iter_mut().for_each(|(_, val)| {
            val.update(delta_ms)
        } )
    }
}

impl From<&Element> for Actor {
    fn from(root: &Element) -> Self {
        let mut components: ActorComponents = HashMap::new();

        for child in root.children() {
            if child.is("CubePhysicsComponent", ACTOR_NS) {
                let component: CubePhysicsComponent = child.into();
                components.insert(component.get_component_id(), Box::new(component));
            }
    
            if child.is("TeapotRenderComponent", ACTOR_NS) {
                let component: TeapotRenderComponent = child.into();
                components.insert(component.get_component_id(), Box::new(component));
            }
        }

        Actor {
            id: 1,
            components
        }
    }
}