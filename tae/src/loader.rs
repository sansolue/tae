use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::archive::{FileStore, get_text};
use crate::world::{DialogueDef, DialogueFile, GameManifest, MapDef, NpcDef, NpcFile};

pub fn load_manifest(store: &FileStore) -> Result<GameManifest> {
    let text = get_text(store, "game.toml")?;
    toml::from_str(text).context("parsing game.toml")
}

pub fn load_map(store: &FileStore, id: &str) -> Result<MapDef> {
    let path = format!("maps/{id}.toml");
    let text = get_text(store, &path)?;
    toml::from_str(text).with_context(|| format!("parsing {path}"))
}

pub fn load_npcs(store: &FileStore) -> Result<HashMap<String, NpcDef>> {
    let path = "entities/npcs.toml";
    if !store.contains_key(path) {
        return Ok(HashMap::new());
    }
    let text = get_text(store, path)?;
    let file: NpcFile = toml::from_str(text).context("parsing entities/npcs.toml")?;
    Ok(file.npc.into_iter().map(|n| (n.id.clone(), n)).collect())
}

pub fn load_dialogues(store: &FileStore) -> Result<HashMap<String, DialogueDef>> {
    let mut map = HashMap::new();
    for key in store.keys() {
        if key.starts_with("dialogue/") && key.ends_with(".toml") {
            let text = get_text(store, key)?;
            let file: DialogueFile =
                toml::from_str(text).with_context(|| format!("parsing {key}"))?;
            for d in file.dialogue {
                map.insert(d.id.clone(), d);
            }
        }
    }
    Ok(map)
}