use crate::world::DialogueDef;

pub struct DialogueState {
    lines: Vec<(String, String)>, // (speaker, text)
    cursor: usize,
    pub then_set_flag: Option<String>,
}

impl DialogueState {
    pub fn start(def: &DialogueDef, then_set_flag: Option<String>) -> Self {
        let lines = def
            .lines
            .iter()
            .map(|l| (l.speaker.clone(), l.text.clone()))
            .collect();
        Self { lines, cursor: 0, then_set_flag }
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
}
