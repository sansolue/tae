use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const TOTAL_SLOTS: usize = 50;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SaveSlot {
    pub name: Option<String>,
    pub timestamp: u64,
    // GameContext fields will be added once fully specified
}

impl SaveSlot {
    pub fn display_name(&self, index: usize) -> String {
        match &self.name {
            Some(n) if !n.is_empty() => n.clone(),
            _ => format!("Slot {}", index + 1),
        }
    }

    pub fn display_timestamp(&self) -> String {
        // Simple unix timestamp display; format properly post-MVP
        format!("Save #{}", self.timestamp % 100_000)
    }
}

pub struct SaveManager {
    pub slots: Vec<Option<SaveSlot>>,
    pub slots_per_page: usize,
    data_dir: PathBuf,
}

impl SaveManager {
    pub fn new(data_dir: PathBuf, slots_per_page: usize) -> Self {
        let mut slots: Vec<Option<SaveSlot>> = vec![None; TOTAL_SLOTS];
        for i in 0..TOTAL_SLOTS {
            let path = slot_path(&data_dir, i);
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(slot) = toml::from_str::<SaveSlot>(&text) {
                    slots[i] = Some(slot);
                }
            }
        }
        Self { slots, slots_per_page, data_dir }
    }

    pub fn page_count(&self) -> usize {
        (TOTAL_SLOTS + self.slots_per_page - 1) / self.slots_per_page
    }

    pub fn page_slots(&self, page: usize) -> &[Option<SaveSlot>] {
        let start = page * self.slots_per_page;
        let end = (start + self.slots_per_page).min(self.slots.len());
        &self.slots[start..end]
    }

    pub fn save(&mut self, slot_idx: usize, name: Option<String>) -> Result<()> {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let slot = SaveSlot { name, timestamp: ts };
        let path = slot_path(&self.data_dir, slot_idx);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, toml::to_string_pretty(&slot)?)?;
        self.slots[slot_idx] = Some(slot);
        Ok(())
    }

    pub fn delete(&mut self, slot_idx: usize) -> Result<()> {
        let path = slot_path(&self.data_dir, slot_idx);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        self.slots[slot_idx] = None;
        Ok(())
    }
}

fn slot_path(data_dir: &Path, index: usize) -> PathBuf {
    data_dir.join(format!("save_{:03}.toml", index + 1))
}
