use std::collections::{HashMap, HashSet};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GameManifest {
    pub title: String,
    pub start_map: String,
    pub tile_size: u32,
    pub window_w: u32,
    pub window_h: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapDef {
    pub id: String,
    pub width: u32,
    pub height: u32,
    /// Row-major tile grid. 0 = floor, 1 = wall.
    pub tiles: Vec<Vec<u32>>,
    #[serde(default)]
    pub entities: Vec<EntityPlacement>,
    #[serde(default)]
    pub triggers: Vec<TriggerDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EntityPlacement {
    pub id: String,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerDef {
    pub x: u32,
    pub y: u32,
    pub action: Action,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    Dialogue {
        id: String,
    },
    MapTransition {
        target_map: String,
        target_x: u32,
        target_y: u32,
    },
    SetFlag {
        flag: String,
    },
    Conditional {
        flag: String,
        then: Box<Action>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct NpcDef {
    pub id: String,
    pub name: String,
    pub glyph: char,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NpcFile {
    pub npc: Vec<NpcDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DialogueDef {
    pub id: String,
    pub lines: Vec<DialogueLine>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DialogueFile {
    pub dialogue: Vec<DialogueDef>,
}

/// Live runtime world state.
pub struct World {
    pub manifest: GameManifest,
    pub current_map: MapDef,
    pub npcs: HashMap<String, NpcDef>,
    pub dialogues: HashMap<String, DialogueDef>,
    pub flags: HashSet<String>,
}

impl World {
    pub fn new(
        manifest: GameManifest,
        start_map: MapDef,
        npcs: HashMap<String, NpcDef>,
        dialogues: HashMap<String, DialogueDef>,
    ) -> Self {
        Self {
            manifest,
            current_map: start_map,
            npcs,
            dialogues,
            flags: HashSet::new(),
        }
    }

    pub fn tile_at(&self, x: u32, y: u32) -> u32 {
        self.current_map
            .tiles
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(1)
    }

    pub fn is_wall(&self, x: u32, y: u32) -> bool {
        self.tile_at(x, y) == 1
    }

    pub fn trigger_at(&self, x: u32, y: u32) -> Option<&TriggerDef> {
        self.current_map.triggers.iter().find(|t| t.x == x && t.y == y)
    }

    pub fn set_flag(&mut self, flag: &str) {
        self.flags.insert(flag.to_string());
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
}