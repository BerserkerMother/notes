use std::mem;

use crossterm::event::{KeyCode, KeyEvent};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use ratatui::widgets::ListState;

use crate::editor_handler::edit_with_vim;

pub struct App {
    pub editor_mode: EditorMode,
    pub logo_position: u16,
    pub should_quit: bool,
    pub mode: AppMode,
    pub tabs: StatefulList,
    pub note_list: NoteList,
    pub search_key_word_related: Vec<usize>,
    matcher: Matcher,
    pub search_query: String,
    is_searched_changed: bool,
}

impl App {
    pub fn new() -> App {
        App {
            editor_mode: EditorMode::None,
            logo_position: 0,
            should_quit: false,
            mode: AppMode::Home,
            tabs: StatefulList::default(),
            note_list: NoteList::default(),
            search_key_word_related: Vec::new(),
            is_searched_changed: false,
            matcher: Matcher::default(),
            search_query: String::new(),
        }
    }

    pub fn handle_press(&mut self, event: KeyEvent) {
        match self.mode {
            AppMode::Home => self.app_handler(event),
            AppMode::NoteView => {
                self.app_handler(event);
                self.note_view_handler(event);
            }
            AppMode::Search => {
                self.search_handler(event);
                self.app_handler(event)
            }
            _ => self.app_handler(event),
        }
    }

    pub fn search_handler(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(key) => self.search_query.push(key),
            KeyCode::Backspace => {
                self.search_query.pop();
                self.is_searched_changed = false
            }
            _ => (),
        };
        self.search_keyword();
    }

    fn app_handler(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Right => self.tabs.next(),
            KeyCode::Left => self.tabs.previous(),
            KeyCode::Char('q') => self.should_quit = true,
            _ => (),
        };
        self.set_app_mode()
    }

    fn note_view_handler(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => self.note_list.previous(),
            KeyCode::Down => self.note_list.next(),
            KeyCode::Char('a') => {
                self.editor_mode = EditorMode::Add;
            }
            KeyCode::Char('e') => self.editor_mode = EditorMode::Edit,
            KeyCode::Char('d') => self.note_list.delete_note(),
            _ => (),
        };
    }

    fn set_app_mode(&mut self) {
        let state = self.tabs.state.selected().unwrap_or(0);
        self.mode = state.into();
    }

    pub fn clibrate_logo_position(&mut self, max_length: u16) {
        self.logo_position %= max_length;
    }

    pub fn get_search_result(&self) -> Vec<&Note> {
        self.search_key_word_related
            .iter()
            .rev()
            .take(5)
            .map(|&index| &self.note_list.notes[index])
            .collect()
    }

    fn search_keyword(&mut self) -> Vec<&Note> {
        let notes = if self.is_searched_changed {
            self.get_search_result()
        } else {
            self.note_list.notes.iter().map(|note| note).collect()
        };
        let mut matched_indices: Vec<_> = self
            .matcher
            .match_all(self.search_query.as_str(), &notes)
            .into_iter()
            .enumerate()
            .collect();
        matched_indices.sort_by_key(|key| key.1);
        let matched_indices = matched_indices.iter().map(|&(index, _)| index).collect();
        // self.is_searched_changed = true;
        self.search_key_word_related = matched_indices;
        if self.search_query.is_empty() {
            self.search_key_word_related = Vec::new();
        }
        self.get_search_result()
    }
}

pub enum AppMode {
    Home,
    NoteView,
    Search,
    Chat,
}
impl From<usize> for AppMode {
    fn from(value: usize) -> Self {
        match value {
            0 => AppMode::Home,
            1 => AppMode::NoteView,
            2 => AppMode::Search,
            3 => AppMode::Chat,
            _ => AppMode::Home,
        }
    }
}
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<&'static str>,
    last_selected: Option<usize>,
}

impl StatefulList {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(1),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(self.items.len() - 1),
        };
        self.state.select(Some(i));
    }
}

impl Default for StatefulList {
    fn default() -> Self {
        StatefulList {
            state: ListState::default(),
            items: vec!["Home", "Notes", "Search", "Chat"],
            last_selected: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct NoteList {
    pub state: ListState,
    pub last_selected: Option<usize>,
    pub notes: Vec<Note>,
}

impl NoteList {
    pub fn add_note(&mut self, title: String, content: String) {
        let note = Note::new(title, content);
        self.notes.push(note);
    }
    fn next(&mut self) {
        if self.notes.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.notes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.notes.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.notes.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => self
                .last_selected
                .unwrap_or(self.notes.len().saturating_sub(1)),
        };
        self.state.select(Some(i));
    }

    pub fn get_selected(&self) -> Option<&Note> {
        if let Some(selected) = self.state.selected() {
            self.notes.get(selected)
        } else {
            None
        }
    }

    /// adds note
    pub fn get_user_note(&mut self) {
        let note = match edit_with_vim(None) {
            Ok(note) => note,
            Err(e) => panic!("failed to parse note with error {e}"),
        };
        let note = note.trim();
        if note.is_empty() {
            return;
        }
        let title_end_index = note.find("\n").unwrap_or(note.len());
        let title = note.get(..title_end_index).unwrap_or_default().to_string();
        let content = note.get(title_end_index..).unwrap_or_default().to_string();
        self.add_note(title, content);
    }

    /// if any note is selected, it deletes it!
    pub fn delete_note(&mut self) {
        if let Some(index) = self.state.selected() {
            self.notes.remove(self.state.selected().unwrap_or_default());
            // set note state after deletion
            if self.notes.is_empty() {
                self.state.select(None);
            } else if self.notes.len() == index {
                self.previous();
            }
        }
    }

    pub fn edit_note(&mut self) {
        let current_note = match self.get_selected() {
            Some(note) => note,
            None => return,
        };
        let text = format!("{}\n{}", current_note.title, current_note.content);
        let note = match edit_with_vim(Some(text.as_str())) {
            Ok(note) => note,
            Err(e) => panic!("failed to parse note with error {e}"),
        };
        let title_end_index = note.find("\n").unwrap_or(note.len());
        let title = note.get(..title_end_index).unwrap_or_default().to_string();
        let content = note.get(title_end_index..).unwrap_or_default().to_string();
        let mut note = Note::new(title, content);
        mem::swap(&mut self.notes[self.state.selected().unwrap()], &mut note); // :)
    }
}

#[derive(Debug, Default)]
pub struct Note {
    id: Option<usize>,
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Note {
        Note {
            id: None,
            title,
            content,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum EditorMode {
    Add,
    Edit,
    None,
}

#[derive(Default)]
struct Matcher {
    matcher: SkimMatcherV2,
}

impl Matcher {
    pub fn match_all(&self, text: &str, notes: &[&Note]) -> Vec<usize> {
        notes
            .iter()
            .map(|&note| {
                self.match_single(text, &format!("{} {}", note.title, note.content)[..])
                    .unwrap_or(0)
            })
            .collect()
    }
    fn match_single(&self, text: &str, pattern: &str) -> Option<usize> {
        let result = if text.len() > pattern.len() {
            self.matcher.fuzzy_match(text, pattern)
        } else {
            self.matcher.fuzzy_match(pattern, text)
        };
        result.map(|sim| sim.max(0) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::Matcher;

    #[test]
    fn matcher() {
        let matcher = Matcher::default();
        dbg!(matcher.match_single("haha this is amazing", "ha"));
    }
}
