use std::{any::Any, cell::RefCell, fmt, rc::Rc};
use minidom::Element;
use flecs_ecs::prelude::*;

use super::{actor::Actor, actor_component::{ActorComponent, ComponentId}, ACTOR_NS};

#[derive(Clone, Debug, Default)]
struct Color(pub f32, pub f32, pub f32, pub f32);

#[derive(Clone, Default, Component)]
pub struct TeapotRenderComponent {
    pub color: Color
}

impl fmt::Debug for TeapotRenderComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color: {:?}", self.color)
    }
}

impl From<&Element> for TeapotRenderComponent {
    fn from(element: &Element) -> Self {
        TeapotRenderComponent {
            color: element
                .get_child("Color", ACTOR_NS)
                .map(|el| el.into())
                .unwrap_or_default()
        }
    }
}

impl From<&Element> for Color {
    fn from(element: &Element) -> Self {
        Color(
            element
                .attr("r")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("g")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("b")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("a")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
        )
    }
}

impl ActorComponent for TeapotRenderComponent {
    fn update(&mut self, delta_ms: i64) {
        todo!()
    }
    
    fn get_component_id(&self) -> ComponentId {
        2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }   
}