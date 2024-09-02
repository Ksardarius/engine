use gem::gem_application::process_management::process::{Process, State};

pub struct SimpleProcess {
    state: State,
    child: Option<Box<dyn Process>>
}

impl SimpleProcess {
    pub fn new() -> Self {
        SimpleProcess {
            state: State::Uninitialized,
            child: None
        }
    }
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