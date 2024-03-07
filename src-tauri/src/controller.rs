use unicode_script::UnicodeScript;

use crate::{cache::Cache, datatypes::{Deck, Decks}, network::GwentAPI};

pub struct GwentController {
    api: GwentAPI,
    cache: Cache
} 

fn contains_chja_characters(s: &str) -> bool {    
    for c in s.chars() {
        if !c.is_alphabetic() {
            continue
        }
        match c.script() {
            unicode_script::Script::Han |
            unicode_script::Script::Katakana |
            unicode_script::Script::Hiragana => {
                println!("Adding {}", s);
                return true
            }
            _ => {
                continue
            }
        }
    }
    println!("Skipping {}", s);
    false
}

impl GwentController {
    pub fn new(api: GwentAPI) -> GwentController {
        GwentController{api, cache: Cache::new()}
    }

    pub async fn get_eastern_decks(&self, offset: u16, limit: u16) -> Decks {
        let probably_cached = self.cache.get_guides_from_cache(offset, limit);
        let json = self.api.get_guides(offset, limit).await.unwrap()
            .json::<Decks>().await.unwrap()
            .guides; 
        let filtered: Vec<Deck> = json.iter() 
            .filter(|deck| -> bool {
                contains_chja_characters(&deck.name)
            }) 
            .cloned()
            .collect();
        Decks{guides: filtered}
    }
}
