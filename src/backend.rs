use anyhow;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{
    io::{stdout, Stdout},
    time::{Duration, Instant},
};

use crate::{
    app::{App, EditorMode},
    ui,
};

pub fn run(tick_rate: Duration) -> anyhow::Result<()> {
    // create app and run it
    let mut app = App::new();
    // for i in 0..100 {
    //     let title = format!("title {i}");
    //     let content = format!("content {i}").repeat(100);
    //     app.note_list.add_note(title, content);
    // }
    let res = run_app(app, tick_rate);
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(mut app: App, tick_rate: Duration) -> anyhow::Result<()> {
    let mut terminal = init_terminal()?;
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout).expect("polling event!") {
            if let Event::Key(key) = event::read().expect("read key event!") {
                if key.kind == KeyEventKind::Press {
                    app.handle_press(key);
                    if app.editor_mode != EditorMode::None {
                        clean_terminal(&mut terminal)?;
                        if app.editor_mode == EditorMode::Add {
                            app.note_list.get_user_note();
                        } else {
                            app.note_list.edit_note();
                        }
                        app.editor_mode = EditorMode::None;
                        terminal = init_terminal()?;
                    }
                    if app.should_quit {
                        break;
                    };
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        app.logo_position += 5; // move main logo forward!
    }

    clean_terminal(&mut terminal)
}
fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    // setup terminal
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn clean_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    Ok(())
}
