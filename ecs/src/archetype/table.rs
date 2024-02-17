// use std::{fmt::Debug, path::Component, rc::Rc};

// #[derive(Debug)]
// struct ComponentVector {
//     data: *const [u8]
// }


// #[derive(Debug)]
// pub struct Table {
//     data: Vec<Vec<i32>>
// }

// impl Table {
//     pub fn new(len: usize) -> Self {
//         Table {
//             data: vec![vec![1]; len]
//         }
//     }

//     pub fn add<T: Component>(&mut self, col: usize, data: T) {
//         self.data[col].push(2);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use core::slice;
//     use std::alloc::{alloc, Layout};

//     use self::{Table, Component};

//     use super::*;

//     #[derive(Debug)]
//     struct Position {
//         x: i32,
//         y: i32
//     }

//     impl Component for Vec<Position> {

//     }

//     impl Component for Position {
        
//     }

//     #[test]
//     fn it_layout() {
//         let bb = Position {
//             x: 1, y: 2
//         };

//         let layout = Layout::for_value(&bb);
//         let s = Layout::array::<Position>(10);

//         unsafe {

//             let mem = alloc(layout).cast::<Position>();
//             let mut arr = Vec::from_raw_parts(mem, 0, 1);
//             arr.push(bb);
//             arr.push(Position {
//                 x: 1, y: 2
//             });

//             println!("{:?}", arr);
            
//         }

    
//         // if let Ok(l) = layout.repeat(10) {
//         //     unsafe {
//         //         let ptr = alloc(layout).cast::<Position>();
    
//         //     }
//         // }
//         // layout.
        


//     }

//     #[test]
//     fn it_works() {
//         let a: Box<dyn Component> = Box::new(Position {
//             x: 1, y: 2
//         });

//         let b = a.as_ref();

//         let c: Vec<&dyn Component> = vec![&Position {
//             x: 1, y: 2
//         }];

//         // let d = b::<Position>;


//         let mut t: Table = Table::new(4);
//         t.add(0, Position {
//             x: 1, y: 2
//         });

//         t.add(0, Position {
//             x: 1, y: 2
//         });

//         t.add(1, Position {
//             x: 1, y: 2
//         });

//         println!("{:?}", t)
//     }
// }