use argh::FromArgs;
use notes::Repository;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(positional)]
    search_query: String,
}

fn main() -> anyhow::Result<()> {
    let cli: Cli = argh::from_env();
    let reposity = Repository::new("notes.db")?;
    // reposity.initialize_db()?;
    let service = notes::NoteService::new(reposity);
    let founed_notes = service.search_notes(cli.search_query.as_ref());
    dbg!(founed_notes);
    Ok(())
}
