use crate::world::DialogueDef;

pub struct DialogueState {
    lines: Vec<(String, String)>, // (speaker, text)
    cursor: usize,
}

impl DialogueState {
    pub fn start(def: &DialogueDef) -> Self {
        let lines = def
            .lines
            .iter()
            .map(|l| (l.speaker.clone(), l.text.clone()))
            .collect();
        Self { lines, cursor: 0 }
    }

    pub fn current(&self) -> Option<(&str, &str)> {
        self.lines
            .get(self.cursor)
            .map(|(s, t)| (s.as_str(), t.as_str()))
    }

    /// Advance to the next line. Returns false when the dialogue is exhausted.
    pub fn advance(&mut self) -> bool {
        if self.cursor + 1 < self.lines.len() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    pub fn is_done(&self) -> bool {
        self.cursor >= self.lines.len()
    }
}
