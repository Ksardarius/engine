#[derive(Debug)]
pub struct ResCache;

impl ResCache {
    pub fn new() -> Self {
        ResCache
    }
}

pub trait HandleResCache {
    fn get_res_cache(&self) -> &ResCache;

    fn init(&self) -> bool {
        true
    }
}