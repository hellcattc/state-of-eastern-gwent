use crate::datatypes::Deck;
use std::collections::hash_map::HashMap;

pub struct Cache {
    guides_hashmap: HashMap<u32, Deck> 
}

impl Cache {
    pub fn new() -> Cache {
        Cache{guides_hashmap: HashMap::default()}
    }

    pub fn get_guides_from_cache(&self, offset: u16, limit: u16) -> Vec<Deck> {
        let mut guides: Vec<Deck> = Vec::new();

        for i in offset..limit {
            if self.guides_hashmap.contains_key(&i.into()) {
                guides.push(self.guides_hashmap.get(&i.into()).unwrap().clone()) 
            }    
        } 

        guides
    }

    pub fn populate_guides_cache(&mut self, offset: u16, limit: u16, guides: Vec<Deck>) {
        for i in offset..limit {
            self.guides_hashmap.insert(i.into(), guides[Into::<usize>::into(i - offset)].clone()); 
        }
    }
}
