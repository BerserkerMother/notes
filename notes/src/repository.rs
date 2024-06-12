use anyhow::{self, Context};
use rusqlite::{self, params, Connection, Result};
use std::{fmt::Display, path::Path};

pub struct Repository {
    db: Connection,
}

impl Repository {
    pub fn new(db_path: impl AsRef<Path>) -> anyhow::Result<Repository> {
        let db = Connection::open(db_path)?;
        Ok(Repository { db })
    }

    pub fn initialize_db(&mut self) -> anyhow::Result<()> {
        self.db.execute(
            "CREATE TABLE IF NOT EXISTS note (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                text TEXT NOT NULL
            )",
            (),
        )?;
        // insert test notes!
        self.insert_test_notes()?;

        Ok(())
    }

    fn insert_test_notes(&mut self) -> anyhow::Result<()> {
        let notes: Vec<Note> = vec![
        (1, "Meeting Notes", "Discuss project milestones and deadlines for Q2. Review pending tasks and assign new ones."),
        (2, "Grocery List", "Milk, Eggs, Bread, Butter, Chicken, Spinach, Tomatoes, Bananas, Apples, Orange Juice"),
        (3, "Workout Plan", "Monday: Chest and Triceps, Tuesday: Back and Biceps, Wednesday: Rest, Thursday: Legs and Shoulders, Friday: Full Body, Saturday: Cardio, Sunday: Rest"),
        (4, "Vacation Ideas", "Visit the Grand Canyon, Explore Yellowstone National Park, Take a trip to New York City, Relax on the beaches of Hawaii, Go skiing in Colorado"),
        (5, "Book Recommendations", "Sapiens by Yuval Noah Harari, Atomic Habits by James Clear, The Lean Startup by Eric Ries, Educated by Tara Westover, The Power of Habit by Charles Duhigg"),
        (6, "Recipe for Spaghetti", "Ingredients: Spaghetti, Marinara Sauce, Ground Beef, Garlic, Onion, Olive Oil, Salt, Pepper. Instructions: Cook spaghetti according to package directions. In a separate pan, sauté garlic and onion in olive oil, add ground beef and cook until browned. Mix in marinara sauce, season with salt and pepper. Combine with cooked spaghetti."),
        (7, "To-Do List", "1. Finish report for work, 2. Call the bank, 3. Schedule dentist appointment, 4. Buy a gift for Sarah's birthday, 5. Clean the garage"),
        (8, "Movie Watchlist", "Inception, The Dark Knight, Interstellar, The Matrix, The Shawshank Redemption, Fight Club, Forrest Gump, The Godfather, Pulp Fiction, The Lord of the Rings"),
        (9, "Learning Goals", "1. Master Python programming, 2. Learn data visualization techniques, 3. Understand machine learning algorithms, 4. Get proficient in SQL and databases, 5. Study cloud computing and AWS services"),
        (10, "Home Improvement Projects", "1. Paint the living room, 2. Install new kitchen cabinets, 3. Replace old windows, 4. Build a deck in the backyard, 5. Update the bathroom fixtures")
    ].into_iter().map(|el| Note::new(Some(el.0), el.1.to_string(), el.2.to_string())).collect();
        self.add(&notes).context("Add test notes!")
    }
    pub async fn insert_test_notes_ai(&self) -> anyhow::Result<()> {
        let notes = self.get_notes()?;
        crate::add(&notes)
            .await
            .context("add test notes to ai engine!")
    }

    pub fn get_notes(&self) -> Result<Vec<Note>> {
        let mut stmt = self.db.prepare("SELECT id, title, text FROM note")?;
        let notes_iter = stmt.query_map([], |row| {
            Ok(Note::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        let mut notes = Vec::new();
        for note in notes_iter {
            notes.push(note?);
        }
        Ok(notes)
    }

    pub fn add(&mut self, notes: &Vec<Note>) -> Result<()> {
        let transaction = self.db.transaction()?;
        {
            let mut stmt =
                transaction.prepare("INSERT INTO note (id, title, text) VALUES (?, ?, ?)")?;
            for note in notes {
                stmt.execute(params![note.id.unwrap(), note.title, note.text])?;
            }
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn get_note(&self, note_id: usize) -> Result<Note> {
        let mut stmt = self
            .db
            .prepare("SELECT id, title, text FROM note WHERE id = ?1")?;
        let mut note_iter = stmt.query_map([note_id], |row| {
            Ok(Note::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        note_iter
            .next()
            .ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)?
            .and_then(|r| Ok(r))
    }

    pub fn delete(&self, note_id: usize) -> Result<Note> {
        let note = self.get_note(note_id)?;
        self.db
            .execute("DELETE FROM note WHERE id = ?1", params![note_id])?;
        Ok(note)
    }

    // Placeholder for the update function if needed
    pub fn update(&self, _note_id: usize, _new_note: Note) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub id: Option<usize>,
    pub title: String,
    pub text: String,
}

impl Note {
    pub fn new(id: Option<usize>, title: String, text: String) -> Note {
        Note { id, title, text }
    }
}

impl From<&Note> for String {
    fn from(value: &Note) -> Self {
        format!("title: {}\n{}", value.title, value.text)
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "title: {}\n{}", self.title, self.text).unwrap();
        Ok(())
    }
}
