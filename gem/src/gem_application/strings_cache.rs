use std::{collections::HashMap, fs::File};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "KEY")]
    pub key: String,
    pub en: String,
    pub lv: String
}

#[derive(Debug)]
pub struct StringsCache(HashMap<String, Record>);

impl StringsCache {
    pub fn new() -> Self {
        StringsCache(HashMap::new())
    }
}

pub trait HandleStringCache {
    fn get_string_cache(&self) -> &StringsCache;
    fn get_string_cache_mut(&mut self) -> &mut StringsCache;

    fn load_strings(&mut self, path: &str) -> bool {
        let strings_cache = self.get_string_cache_mut();

        let file = File::open(path).unwrap();

        let mut rdr = csv::Reader::from_reader(file);
        let s: HashMap<_, _> = rdr.deserialize().map(|el| {
            let r: Record = el.unwrap();
            (r.key.clone(), r)
        }).collect();

        strings_cache.0 = s;

        true
    }

    fn get_string(&self, key: &str) -> Option<&str> {
        if let Some(val) = self.get_string_cache().0.get(key) {
            Some(val.en.as_str())
       } else {
            None
       }
    }
}