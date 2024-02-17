mod table;

use std::{collections::HashMap, fmt::Debug};
// use table::Table;


type ComponentId = i64;

#[derive(Debug)]
struct Class(Vec<ComponentId>);

type ArchetypeId = i64;

#[derive(Debug)]
struct ArchetypeEdge<'a> {
    add: &'a Archetype<'a>,
    remove: &'a Archetype<'a>,
}

#[derive(Debug)]
pub struct Archetype<'a> {
    id: ArchetypeId,
    class: Class,
    // columns: Table,
    edges: HashMap<ComponentId, ArchetypeEdge<'a>>,
}

impl Archetype<'_> {
    pub fn new(len: usize) -> Self {
        Archetype {
            id: 1,
            class: Class(vec![1]),
            // columns: Table::new(len),
            edges: HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use self::Archetype;

    use super::*;

    #[test]
    fn it_works() {
        let a: Archetype = Archetype::new(3);
        println!("{:?}", a)
    }
}
