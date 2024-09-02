#[derive(Debug)]
pub struct ResCache;

impl ResCache {
    pub fn new() -> Self {
        ResCache
    }
}

pub trait HandleResCache {
    fn get_res_cache(&mut self) -> &ResCache;

    fn init(&mut self) -> bool {
        true
    }
}