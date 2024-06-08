use std::borrow::BorrowMut;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{self, Text},
    widgets::{Block, BorderType, Borders, List, Padding, Paragraph, Tabs, Wrap},
    Frame,
};

#[allow(clippy::wildcard_imports)]
use crate::app::App;
use crate::{app::AppMode, widgets};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(5),
        Constraint::Percentage(100),
        Constraint::Length(3),
    ])
    .split(f.size());
    app.clibrate_logo_position(chunks[1].width); // calibrate the position to not go out of area width
    f.render_widget(Block::new().on_cyan(), f.size());
    render_tabs(f, app, chunks[0]);
    render_app(f, app, chunks[1]);
    draw_footer(f, chunks[2]);
}
pub fn render_app(f: &mut Frame, app: &mut App, area: Rect) {
    match app.mode {
        AppMode::Home => {
            f.render_widget(
                widgets::NotesLogo::default().set_position(app.logo_position),
                area,
            );
        }
        AppMode::NoteView => {
            render_note_view(f, app, area);
        }
        AppMode::Search => render_search(f, app, area),
        _ => (),
    }
}
pub fn render_search(f: &mut Frame, app: &mut App, area: Rect) {
    // divide the layout
    let vertical =
        Layout::vertical([Constraint::Percentage(100), Constraint::Length(2)]).split(area); // small area to add view note keys(add, edit, delete)
    let helpers = Paragraph::new(app.search_query.as_str())
        .on_light_yellow()
        .blue();
    f.render_widget(helpers, vertical[1]);
    let chunks = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vertical[0]);
    let titles = List::new(
        app.get_search_result()
            .iter()
            .map(|note| Text::raw(note.title.as_str()).yellow()),
    )
    .block(
        Block::bordered()
            .border_type(BorderType::Double)
            .yellow()
            .title("title"),
    );
    f.render_widget(titles, chunks[0]);
    let selected_note = app.note_list.get_selected();
    let content = if let Some(note) = selected_note {
        note.content.as_str()
    } else {
        "Search Something!"
    };
    let content = Paragraph::new(content).wrap(Wrap { trim: true }).block(
        Block::bordered()
            .border_type(BorderType::Double)
            .yellow()
            .title("content"),
    );
    f.render_widget(content, chunks[1]);
}

pub fn render_note_view(f: &mut Frame, app: &mut App, area: Rect) {
    // divide the layout
    let vertical =
        Layout::vertical([Constraint::Percentage(100), Constraint::Length(1)]).split(area); // small area to add view note keys(add, edit, delete)
    let helpers = Paragraph::new("Press a: add, e: edit, d: delete")
        .on_light_yellow()
        .blue();
    f.render_widget(helpers, vertical[1]);
    let chunks = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vertical[0]);
    let titles = List::new(
        app.note_list
            .notes
            .iter()
            .map(|note| Text::raw(note.title.as_str()).yellow()),
    )
    .highlight_symbol("=>")
    .highlight_style(Style::default().red())
    .block(
        Block::bordered()
            .border_type(BorderType::Double)
            .yellow()
            .title("title"),
    );
    f.render_stateful_widget(titles, chunks[0], app.note_list.state.borrow_mut());
    let selected_note = app.note_list.get_selected();
    let content = if let Some(note) = selected_note {
        note.content.as_str()
    } else {
        "Select Note to Show!"
    };
    let content = Paragraph::new(content).wrap(Wrap { trim: true }).block(
        Block::bordered()
            .border_type(BorderType::Double)
            .yellow()
            .title("content"),
    );
    f.render_widget(content, chunks[1]);
}

pub fn render_tabs(f: &mut Frame, app: &mut App, area: Rect) {
    // Calculate padding to center tabs
    let remaining_width = area.width as isize - 30;
    let padding = (remaining_width / 2).max(0) as u16;
    let tabs = app
        .tabs
        .items
        .iter()
        .map(|&title| text::Line::from(title).centered())
        .collect::<Tabs>()
        .block(
            Block::new()
                .borders(Borders::BOTTOM)
                .border_type(BorderType::QuadrantInside)
                .title("Note Menu".yellow())
                .title_alignment(Alignment::Center)
                .title_style(Style::default().bold())
                .light_yellow()
                .padding(Padding::new(padding, padding, 1, 0)),
        )
        .highlight_style(Style::default().blue())
        .select(app.tabs.state.selected().unwrap_or(0));
    f.render_widget(tabs, area)
}
pub fn draw_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Take a Great Care of Yourself😊")
        .style(Style::default().yellow().bold())
        .block(
            Block::new()
                .style(Style::new().black())
                .padding(Padding::top(1)),
        )
        .centered();
    f.render_widget(footer, area)
}
