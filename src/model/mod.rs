use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Sorting {
    Name,
    #[default]
    Initiative,
    ArmorClass,
    Health,
    Piece,
}

impl Sorting {
    pub fn next(self) -> Self {
        match self {
            Sorting::Name => Sorting::Initiative,
            Sorting::Initiative => Sorting::ArmorClass,
            Sorting::ArmorClass => Sorting::Health,
            Sorting::Health => Sorting::Piece,
            Sorting::Piece => Sorting::Name,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Sorting::Name => Sorting::Piece,
            Sorting::Initiative => Sorting::Name,
            Sorting::ArmorClass => Sorting::Initiative,
            Sorting::Health => Sorting::ArmorClass,
            Sorting::Piece => Sorting::Health,
        }
    }
}

impl Display for Sorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Sorting::Name => "Name",
            Sorting::Initiative => "Initiative",
            Sorting::ArmorClass => "ArmorClass",
            Sorting::Health => "Health",
            Sorting::Piece => "Piece",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Entities {
    pub e: Vec<Entity>,
}

impl Entities {
    pub fn sort(&mut self, s: &Sorting) {
        match s {
            Sorting::Name => self.e.sort_by(|a, b| a.name.cmp(&b.name)),
            Sorting::Initiative => self.e.sort_by(|a, b| a.initiative.cmp(&b.initiative)),
            Sorting::ArmorClass => self.e.sort_by(|a, b| a.armor_class.cmp(&b.armor_class)),
            Sorting::Health => self.e.sort_by(|a, b| a.health.cmp(&b.health)),
            Sorting::Piece => self.e.sort_by(|a, b| a.piece.cmp(&b.piece)),
        }
    }

    pub fn test() -> Self {
        let e1 = Entity {
            name: "Name1".to_string(),
            initiative: "12".to_string(),
            piece: "white".to_string(),
            armor_class: "14".to_string(),
            health: "56".to_string(),
        };
        let e2 = Entity {
            name: "Name2".to_string(),
            initiative: "13".to_string(),
            piece: "black".to_string(),
            armor_class: "15".to_string(),
            health: "52".to_string(),
        };
        Entities { e: vec![e1, e2] }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Entity {
    pub name: String,
    pub initiative: String,
    /// physical piece
    pub piece: String,
    pub armor_class: String,
    pub health: String,
}

impl Entity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.armor_class.clear();
        self.name.clear();
        self.health.clear();
        self.piece.clear();
        self.initiative.clear();
    }
}
