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
        let _probably_cached = self.cache.get_guides_from_cache(offset, limit);
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

    pub async fn get_specific_deck(&self, id: &str) {
        let parsed_id: u32 = id
            .trim()                       // optional: remove whitespace
            .parse::<u32>()               // tries to parse as base‑10 integer
            .expect("Invalid deck ID");

        let json = self.api.get_deck(parsed_id).await.unwrap()
            .json::<Deck>();
    }
}
