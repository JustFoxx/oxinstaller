use std::io;
use std::io::{stdout, BufWriter, Stdout};

use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::terminal::App;

mod terminal;

type IoResult<T> = io::Result<T>;

fn main() -> IoResult<()> {
    let subscriber = tracing_subscriber::fmt().finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut app = open_app()?;

    loop {
        app.event_loop()?;
        app.main_loop()?;
    }
}

pub fn open_app() -> IoResult<App<BufWriter<Stdout>, CrosstermBackend<Stdout>>> {
    let out = BufWriter::new(stdout());
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(App::new(out, terminal))
}
