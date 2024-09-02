use std::{any::Any, borrow::Borrow, cell::RefCell, fmt, rc::Rc};
use minidom::Element;
use flecs_ecs::prelude::*;

use super::{actor::Actor, actor_component::{ActorComponent, ComponentId}, teapot_render_component::TeapotRenderComponent, ACTOR_NS};

#[derive(Clone, Debug, Default)]
struct Position(pub i32, pub i32, pub i32);
#[derive(Clone, Debug, Default)]
struct Dimensions(pub i32, pub i32, pub i32);
#[derive(Clone, Debug, Default)]
struct Orientation(i32);

#[derive(Clone, Debug, Default)]
struct InitialTransform {
    pub position: Position,
    pub orientation: Orientation,
}

#[derive(Clone, Debug, Default)]
struct Shape {
    pub dimensions: Dimensions,
}

#[derive(Clone, Component, Debug)]
pub struct CubePhysicsComponent {
    pub density: String,
    physics_material: String,
    initial_transform: InitialTransform,
    shape: Shape,
}

impl From<&Element> for CubePhysicsComponent {
    fn from(element: &Element) -> Self {
        CubePhysicsComponent {
            density: element
                .get_child("Density", ACTOR_NS)
                .map(|el| el.text())
                .unwrap_or_default(),
            initial_transform: element
                .get_child("InitialTransform", ACTOR_NS)
                .map(|el| el.into())
                .unwrap_or_default(),
            physics_material: element
                .get_child("PhysicsMaterial", ACTOR_NS)
                .map(|el| el.text())
                .unwrap_or_default(),
            shape: element
                .get_child("Shape", ACTOR_NS)
                .map(|el| el.into())
                .unwrap_or_default(),
        }
    }
}

impl From<&Element> for InitialTransform {
    fn from(element: &Element) -> Self {
        InitialTransform {
            position: element
                .get_child("Position", ACTOR_NS)
                .map(|el| el.into())
                .unwrap_or_default(),
            orientation: element
                .get_child("Orientation", ACTOR_NS)
                .map(|el| el.into())
                .unwrap_or_default(),
        }
    }
}

impl From<&Element> for Shape {
    fn from(element: &Element) -> Self {
        Shape {
            dimensions: element
                .get_child("Dimensions", ACTOR_NS)
                .map(|el| el.into())
                .unwrap_or_default(),
        }
    }
}

impl From<&Element> for Position {
    fn from(element: &Element) -> Self {
        Position(
            element
                .attr("x")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("y")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("z")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
        )
    }
}

impl From<&Element> for Orientation {
    fn from(element: &Element) -> Self {
        Orientation(
            element
                .attr("degrees")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
        )
    }
}

impl From<&Element> for Dimensions {
    fn from(element: &Element) -> Self {
        Dimensions(
            element
                .attr("x")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("y")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
            element
                .attr("z")
                .map(|a| a.parse().unwrap_or_default())
                .unwrap_or_default(),
        )
    }
}

impl ActorComponent for CubePhysicsComponent {
    fn update(&mut self, delta_ms: i64) {
        // let _component = self.actor.as_ref().unwrap().borrow_mut().get_component::<TeapotRenderComponent>(2).unwrap();
    }
    
    fn get_component_id(&self) -> ComponentId {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn init(&mut self) {}
    
    fn post_init(&mut self) {}
}
