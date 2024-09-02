use std::{cell::RefCell, rc::Rc};

use gem::gem_application::{process_management::process::{Process, State}, strings_cache::HandleStringCache, GemApplication};
use log::{error, info};
use my_game::MyGemApplication;
use process::{DelayProcess, SimpleProcess};
use simple_logger::SimpleLogger;

mod my_game;
mod process;

pub fn init_game() -> usize {
    let mut gem_app = MyGemApplication::new();

    let process = SimpleProcess::new();

    let mut delay_process = DelayProcess::new(5.0);
    delay_process.attach_child(Box::new(process));

    gem_app.process_manager.attach_process(Rc::new(RefCell::new(delay_process)));

    // init log
    SimpleLogger::new().init().unwrap();

    // init options
    gem_app.options.init("/Users/morlovs/Projects/rust/engine/my-game/lib/config.toml");
    let config = gem_app.options.get_config().unwrap();

    // init application layer
    if !gem_app.init_instance(config.screen_size.x, config.screen_size.y, "/Users/morlovs/Projects/rust/engine/my-game/lib/locales/localisation.csv") {
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