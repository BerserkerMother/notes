use std::cmp::Reverse;

use anyhow::Context;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::repository::{Note, Repository};

pub struct NoteService {
    db_manager: Repository,
    matcher: Matcher,
}

impl NoteService {
    pub fn new(db_manager: Repository) -> NoteService {
        NoteService {
            db_manager,
            matcher: Matcher::default(),
        }
    }

    pub fn add_note(&mut self, note: Note) -> anyhow::Result<()> {
        let notes = vec![note];
        self.db_manager.add(notes).context("service add note")
    }

    pub fn get_note(&mut self, id: usize) -> anyhow::Result<Note> {
        self.db_manager.get_note(id).context("service get one note")
    }

    pub fn get_all(&self) -> anyhow::Result<Vec<Note>> {
        self.db_manager.get_notes().context("service get all notes")
    }

    pub fn delete_note(&mut self, id: usize) -> anyhow::Result<Note> {
        self.db_manager.delete(id).context("service delete note")
    }

    pub fn search_notes(&self, query: &str) -> anyhow::Result<Vec<(Note, usize)>> {
        let notes = self.get_all()?;
        // use note refs so it can be passed from another function later, probably a cache
        let notes_refs: Vec<&Note> = notes.iter().map(|note| note).collect();

        let notes_scores = self.matcher.match_all(query, &notes_refs[..]);
        let mut selected_notes: Vec<(Note, usize)> = notes
            .into_iter()
            .enumerate()
            .filter(|&(index, _)| notes_scores[index].is_some())
            .map(|(index, note)| (note, notes_scores[index].unwrap()))
            .collect();
        selected_notes.sort_by_key(|&(_, score)| Reverse(score));
        Ok(selected_notes)
    }

    pub fn semantic_search_notes(&self, query: &str) -> Vec<Note> {
        todo!()
    }
}

#[derive(Default)]
struct Matcher {
    matcher: SkimMatcherV2,
}

impl Matcher {
    pub fn match_all(&self, text: &str, notes: &[&Note]) -> Vec<Option<usize>> {
        notes
            .iter()
            .map(|&note| self.match_single(text, &format!("{} {}", note.title, note.text)[..]))
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
