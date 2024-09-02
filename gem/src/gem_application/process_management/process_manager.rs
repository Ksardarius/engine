use std::{cell::{Cell, RefCell}, rc::{Rc, Weak}};

use super::process::{Process, State};

#[derive(Debug)]
pub struct ProcessManager {
    process_list: Vec<Rc<RefCell<dyn Process>>>
}

impl ProcessManager {
    pub fn new() -> ProcessManager {
        ProcessManager {
            process_list: vec![]
        }
    }

    pub fn update_processes(&mut self, delta_s: f64) -> i32 {
        let mut success_count = 0;
        let mut fail_count = 0;

        let mut to_add: Vec<Rc<RefCell<dyn Process>>> = vec![];

        self.process_list.retain(|process_ref| {
            let delete = {
                let mut process = process_ref.borrow_mut();

                if let State::Uninitialized = process.get_state() {
                    process.on_init();
                }

                if let State::Running = process.get_state() {
                    process.on_update(delta_s);
                }

                if process.is_dead() {
                    match process.get_state() {
                        State::Succeeded => {
                            process.on_success();
                            if let Some(child) = process.remove_child() {
                                to_add.push(child.wrap_in_refcell());
                            } else {
                                success_count += 1;
                            };
                        },
                        State::Failed => {
                            process.on_fail();
                            fail_count += 1;

                        },
                        State::Aborted => {
                            process.on_abort();
                            fail_count += 1;
                        }
                        _ => {}
                    };
                    return true
                } else {
                    false
                }
            };

            !delete
        });

        while let Some(e) = to_add.pop() {
            self.attach_process(e);
        }

        success_count << 16 | fail_count
    }

    pub fn attach_process(&mut self, process: Rc<RefCell<dyn Process>>) -> Weak<RefCell<dyn Process>> {
        self.process_list.push(Rc::clone(&process));
        Rc::downgrade(&process)
    }

    pub fn abort_all_processes(&self, immediate: bool) {

    }

    pub fn get_process_count(&self) -> usize {
        self.process_list.len()
    }
}

pub struct SimpleProcess {
    state: State,
    child: Option<Box<dyn Process>>
}

impl Process for SimpleProcess {
    fn get_state(&self) -> State {
        self.state
    }

    fn change_state(&mut self, state: State) {
        self.state = state;
    }
    
    fn on_update(&mut self, delta_s: f64) {
        println!("Update process");
    }

    fn get_child(&mut self) -> &mut Option<Box<dyn Process>> {
        &mut self.child
    }
}

pub struct DelayProcess {
    state: State,
    child: Option<Box<dyn Process>>,

    time_to_delay: f64,
    time_delayed_so_far: f64
}

impl DelayProcess {
    pub fn new(time_to_delay: f64) -> DelayProcess {
        DelayProcess {
            state: State::Uninitialized,
            child: None,

            time_to_delay,
            time_delayed_so_far: 0.0
        }
    }
}

impl Process for DelayProcess {
    fn get_state(&self) -> State {
        self.state
    }

    fn change_state(&mut self, state: State) {
        self.state = state;
    }
    
    fn on_update(&mut self, delta_s: f64) {
        println!("Delay tick");
        self.time_delayed_so_far += delta_s;
        if self.time_delayed_so_far >= self.time_to_delay {
            self.succeed();
        }
    }

    fn get_child(&mut self) -> &mut Option<Box<dyn Process>> {
        &mut self.child
    }
}

#[cfg(test)]
mod tests {
    use std::process;

    use log::info;

    use crate::gem_application::process_management::process::State;

    use super::*;

    #[test]
    fn it_works() {
        let mut process = SimpleProcess {
            state: State::Uninitialized,
            child: None
        };

        let child_process = SimpleProcess {
            state: State::Uninitialized,
            child: None
        };

        let child_process2 = SimpleProcess {
            state: State::Uninitialized,
            child: None
        };

        let mut manager = ProcessManager {
            process_list: vec![]
        };

        let proc = manager.attach_process(Rc::new(RefCell::new(process)));
        let proc2 = manager.attach_process(Rc::new(RefCell::new(child_process)));

        manager.update_processes(1.0);

        // proc2.upgrade().unwrap().attach_child(Rc::new(child_process2));

        proc2.upgrade().unwrap().borrow_mut().attach_child(Box::new(child_process2));

        let data = proc2.upgrade().unwrap();



        println!("{data:?}");

        assert_eq!(4, 4);
    }

    #[test]
    fn delay_works() {
        let process = SimpleProcess {
            state: State::Uninitialized,
            child: None
        };

        let mut delay_process = DelayProcess::new(10.0);
        delay_process.attach_child(Box::new(process));


        let mut manager = ProcessManager {
            process_list: vec![]
        };

        let _proc = manager.attach_process(Rc::new(RefCell::new(delay_process)));

        manager.update_processes(1.0);
        manager.update_processes(5.0);
        manager.update_processes(5.0);
        manager.update_processes(5.0);

        // proc2.upgrade().unwrap().attach_child(Rc::new(child_process2));

        // proc2.upgrade().unwrap().borrow_mut().attach_child(Box::new(child_process2));

        // let data = proc.upgrade().unwrap();



        // println!("{data:?}");

        assert_eq!(4, 4);
    }
}