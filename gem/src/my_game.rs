use res_cache::{HandleResCache, ResCache};
use script_manager::{HandleScriptManager, ScriptManager};

use crate::gem_application::{options::Options, process_management::process_manager::ProcessManager, strings_cache::{HandleStringCache, StringsCache}, GemApplication, HandleWindow};

#[derive(Debug)]
pub struct MyGemApplication {
    pub res_cache: ResCache,
    str_cache: StringsCache,
    pub options: Options,
    script_manager: ScriptManager,
    pub process_manager: ProcessManager
}

impl MyGemApplication {
    pub fn new() -> Self {
        MyGemApplication {
            res_cache: ResCache::new(),
            str_cache: StringsCache::new(),
            options: Options::new(),
            script_manager: ScriptManager::new(),
            process_manager: ProcessManager::new()
        }
    }
}



impl GemApplication for MyGemApplication {
    fn create_game_and_view(&mut self) -> bool {
        true
    }
}

impl HandleResCache for MyGemApplication {
    fn get_res_cache(&self) -> &ResCache {
        &self.res_cache
    }
}

impl HandleScriptManager for MyGemApplication {
    fn get_script_manager(&mut self) -> &mut ScriptManager {
        &mut self.script_manager
    }
}

impl HandleStringCache for MyGemApplication {
    fn get_string_cache(&self) -> &StringsCache {
        &self.str_cache
    }

    fn get_string_cache_mut(&mut self) -> &mut StringsCache {
        &mut self.str_cache
    }
}

impl HandleWindow for MyGemApplication {
    fn on_update(&mut self, delta_s: f64) {
        self.process_manager.update_processes(delta_s);
    }
}