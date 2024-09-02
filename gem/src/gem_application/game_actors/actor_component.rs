use std::{any::Any, cell::RefCell, rc::Rc};
use core::fmt::Debug;

use super::actor::Actor;

pub type ComponentId = i64;


pub trait ActorComponent: Debug {
    fn get_component_id(&self) -> ComponentId;
    fn init(&mut self) {}
    fn post_init(&mut self) {}
    fn update(&mut self, delta_ms: i64);


    fn as_any(&self) -> &dyn Any; 
    fn as_any_mut(&mut self) -> &mut dyn Any; 
}