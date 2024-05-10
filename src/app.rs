use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;

pub struct App<'a> {
    pub logo_position: u16,
    title: &'a str,
    enhanced_graphics: bool,
    pub should_quit: bool,
    pub mode: AppMode,
    pub tabs: StatefulList,
    pub notes: NoteList,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App {
        App {
            logo_position: 0,
            title,
            enhanced_graphics,
            should_quit: false,
            mode: AppMode::Home,
            tabs: StatefulList::default(),
            notes: NoteList::default(),
        }
    }
    pub fn handle_press(&mut self, event: KeyEvent) {
        match self.mode {
            AppMode::Home => self.app_handler(event),
            AppMode::NoteView => self.app_handler(event),
            _ => self.app_handler(event),
        }
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
            KeyCode::Right => self.tabs.next(),
            KeyCode::Left => self.tabs.previous(),
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
}

pub enum AppMode {
    Home,
    NoteView,
    NoteCreate,
    Delete,
    Search,
    Chat,
}
impl From<usize> for AppMode {
    fn from(value: usize) -> Self {
        match value {
            0 => AppMode::Home,
            1 => AppMode::NoteView,
            2 => AppMode::NoteCreate,
            3 => AppMode::Delete,
            4 => AppMode::Search,
            5 => AppMode::Chat,
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
            items: vec!["Home", "Notes", "Search", "Chat", "Add", "Delete"],
            last_selected: None,
        }
    }
}

#[derive(Debug, Default)]
struct NoteList {
    note_state: ListState,
    last_selected: Option<usize>,
    titles: HashMap<usize, String>,
    contents: HashMap<usize, String>,
}

impl NoteList {
    fn add_note(&mut self, id: usize, title: String, content: String) {
        self.titles.insert(id, title);
        self.contents.insert(id, content);
    }
}
