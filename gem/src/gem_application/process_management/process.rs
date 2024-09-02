use std::{cell::RefCell, rc::Rc, fmt::Debug};

use log::{log, trace, warn};

#[derive(Clone, Copy, Debug)]
pub enum State {
    Uninitialized,
    Removed,
    Running,
    Paused,
    Succeeded,
    Failed,
    Aborted
}

pub trait HelperTrait {
    fn wrap_in_refcell(self: Box<Self>) -> Rc<RefCell<dyn Process>>;
}
impl<T: Process + 'static> HelperTrait for T {
    fn wrap_in_refcell(self: Box<Self>) -> Rc<RefCell<dyn Process>> {
        Rc::new(RefCell::new(*self))
    }
}

pub trait Process: HelperTrait {
    fn change_state(&mut self, state: State);
    fn get_state(&self) -> State;

    fn on_init(&mut self) {
        self.change_state(State::Running);
    }

    fn on_update(&mut self, delta_s: f64);
    fn on_success(&mut self) {}
    fn on_fail(&mut self) {}
    fn on_abort(&mut self) {}

    fn succeed(&mut self) {
        assert!(self.is_alive());
        self.change_state(State::Succeeded);
    }

    fn fail(&mut self) {
        assert!(self.is_alive());
        self.change_state(State::Failed);
    }

    fn pause(&mut self) {
        match self.get_state() {
            State::Running => {
                self.change_state(State::Paused)
            }
            _ => warn!("Attempting to pause a process that isn't running")
        };
    }

    fn unpause(&mut self) {
        match self.get_state() {
            State::Paused => {
                self.change_state(State::Running)
            }
            _ => warn!("Attempting to unpause a process that isn't paused")
        };
    }

    fn is_alive(&self) -> bool {
        let state = self.get_state();
        match state {
            State::Running | State::Paused => true,
            _ => false
        }
    }

    fn is_dead(&self) -> bool {
        let state = self.get_state();
        match state {
            State::Succeeded | State::Failed | State::Aborted => true,
            _ => false
        }
    }

    fn is_removed(&self) -> bool {
        let state = self.get_state();
        match state {
            State::Removed => true,
            _ => false
        }
    }

    fn is_paused(&self) -> bool {
        let state = self.get_state();
        match state {
            State::Paused => true,
            _ => false
        }
    }

    fn get_child(&mut self) -> &mut Option<Box<dyn Process>>;

    fn attach_child(&mut self, process: Box<dyn Process>) {
        let child = self.get_child();
        if let None = child {
            child.replace(process);
        }   
    }

    fn remove_child(&mut self) -> Option<Box<dyn Process>> {
        self.get_child().take()
    }
}

impl Debug for dyn Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State: {:?}", self.get_state())
    }
}

pub struct SimpleProcess {
    pub state: State,
    pub child: Option<Box<dyn Process>>
}

impl Process for SimpleProcess {
    fn get_state(&self) -> State {
        self.state
    }

    fn change_state(&mut self, state: State) {
        self.state = state;
    }
    
    fn on_update(&mut self, delta_s: f64) {
        trace!("Update process");
    }
    
    fn get_child(&mut self) -> &mut Option<Box<dyn Process>> {
        &mut self.child
    }
}

#[cfg(test)]
mod tests {
    use std::process;

    use log::info;

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

        process.on_init();
        process.attach_child(Box::new(child_process));

        // info!("{process:?}");

        assert_eq!(4, 4);
    }
}