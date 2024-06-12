use argh::FromArgs;
use notes::Repository;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(positional)]
    search_query: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: Cli = argh::from_env();
    let mut reposity = Repository::new("notes.db")?;
    // reposity.initialize_db()?;
    // reposity.insert_test_notes_ai().await?;
    let service = notes::NoteService::new(reposity);
    // let founed_notes = service.search_notes(cli.search_query.as_ref());
    // let search_res = notes::search(cli.search_query.as_ref()).await?;
    let search_res = service.search_ai(cli.search_query.as_ref()).await?;
    dbg!(search_res);
    Ok(())
}
