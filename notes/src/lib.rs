mod app;
mod backend;
mod editor_handler;
mod repository;
mod service;
// mod handler;
mod ai_embedding;
mod ui;
mod widgets;

pub use ai_embedding::{add, search};
pub use backend::run;
pub use repository::Repository;
pub use service::NoteService;
