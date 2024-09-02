use std::{cell::RefCell, rc::Rc};

use gem_application::{process_management::{process::{Process, SimpleProcess, State}, process_manager::DelayProcess}, strings_cache::HandleStringCache, GemApplication};
use log::{error, info};
use my_game::MyGemApplication;
use simple_logger::SimpleLogger;

pub mod gem_application;
pub mod my_game;

pub fn init_game() -> usize {
    let mut gem_app = MyGemApplication::new();

    let process = SimpleProcess {
        state: State::Uninitialized,
        child: None
    };

    let mut delay_process = DelayProcess::new(5.0);
    delay_process.attach_child(Box::new(process));

    gem_app.process_manager.attach_process(Rc::new(RefCell::new(delay_process)));

    // init log
    SimpleLogger::new().init().unwrap();

    // init options
    gem_app.options.init("/Users/morlovs/Projects/rust/engine/gem/lib/config.toml");
    let config = gem_app.options.get_config().unwrap();

    // init application layer
    if !gem_app.init_instance(config.screen_size.x, config.screen_size.y, "/Users/morlovs/Projects/rust/engine/gem/lib/locales/localisation.csv") {
        error!("Application layer initialization error");
        return 0
    }

    main_loop();
    shutdown();

    let a = gem_app.get_string("LABEL_TEXT").unwrap();


    info!(target: "gem_events", "Commencing yak shaving for {gem_app:?} {a:?}");
    
    gem_app.get_exit_code()
}

fn main_loop() {

}

fn shutdown() {

}

fn main() {
    init_game();
}