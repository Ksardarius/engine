#[derive(Debug)]
pub struct ScriptManager;

impl ScriptManager {
    pub fn new() -> Self {
        ScriptManager
    }
}

pub trait HandleScriptManager {
    fn get_script_manager(&mut self) -> &mut ScriptManager;

    fn load(&mut self) -> bool {
        true
    }
}
