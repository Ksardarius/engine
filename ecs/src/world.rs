use crate::archetype::Archetype;

#[derive(Debug)]
pub struct World<'a> {
    archetype: Archetype<'a>
}

#[cfg(test)]
mod tests {
    use self::World;

    use super::*;

    #[test]
    fn it_works() {
        let a: World = World {
            archetype: Archetype::new(3)
        };

        println!("{:?}", a)
    }
}