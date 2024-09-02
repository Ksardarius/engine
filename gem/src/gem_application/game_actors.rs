mod actor;
mod actor_component;
mod actor_factory;
mod actor_prefab;
mod cube_physics_component;
mod teapot_render_component;

use flecs_ecs::prelude::*;

pub const ACTOR_NS: &'static str = "actor";

#[cfg(test)]
mod tests {
    use actor_factory::ActorFactory;
    use cube_physics_component::CubePhysicsComponent;
    use flecs_ecs::core::World;
    use teapot_render_component::TeapotRenderComponent;

    use super::*;

    #[test]
    fn it_works() {
        let mut factory = ActorFactory::new(0);
        let actor =
            factory.create_actor("/Users/morlovs/Projects/rust/engine/gem/lib/actors/actor1.xml");
        let actor2 =
            factory.create_actor("/Users/morlovs/Projects/rust/engine/gem/lib/actors/actor2.xml");
        if let Some(component) = actor.get_component::<CubePhysicsComponent>(1) {
            println!("component found: {component:?}");
        } else {
            println!("component not found");
        }

        println!("actor: {actor:?}; actor2: {actor2:?}");
        assert_eq!(4, 4);
    }

    #[test]
    fn it_use_ecs() {
        let world = World::new();
        world
            .component::<CubePhysicsComponent>()
            .add_trait::<(flecs::OnInstantiate, flecs::Override)>();

        world
            .component::<TeapotRenderComponent>()
            .add_trait::<(flecs::OnInstantiate, flecs::Override)>();

        let mut factory = ActorFactory::new(0);
        let prefab1 = factory.create_prefab(
            "/Users/morlovs/Projects/rust/engine/gem/lib/actors/actor1.xml",
            &world,
        );
        // let prefab2 = factory.create_prefab(
        //     "/Users/morlovs/Projects/rust/engine/gem/lib/actors/actor2.xml",
        //     &world,
        // );

        // Create a prefab instance
        let inst = world.entity_named("my_spaceship").is_a_id(prefab1);
        let _inst2 = world.entity_named("my_spaceship_2").is_a_id(prefab1);

        inst.try_get::<&CubePhysicsComponent>(|d_inst| {
            println!("{:?}", d_inst);
        });

        world.each_entity::<&CubePhysicsComponent>(|entity, d| {
            println!("{}: defence: {}", entity.path().unwrap(), d.density);
        });

        world.each_entity::<(&CubePhysicsComponent, &TeapotRenderComponent)>(|entity, (cube, teapot)| {
            println!("{}+{}: aaaaa: {}", entity.name(), entity.path().unwrap(), cube.density);
        });

        // println!("actor: {actor:?}; actor2: {actor2:?}");
        assert_eq!(4, 4);
    }
}
