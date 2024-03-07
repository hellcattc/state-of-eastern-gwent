use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Decks {
    pub guides: Vec<Deck>
}

#[derive(Deserialize, Serialize)]
pub struct Deck {
    pub id: u32,
    pub name: String, 
    pub created: String
}

impl Clone for Deck {
    fn clone(&self) -> Self {
        Deck{
            id: self.id.clone(),
            name: self.name.clone(),
            created: self.created.clone()
        }
    }
}
