use std::io::Write;
use std::process::exit;
use std::time::Duration;

use crossterm::event::{poll, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{event, ExecutableCommand};
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};

use crate::IoResult;

pub struct App<T: Write, B: Backend> {
    out: T,
    terminal: Terminal<B>,
}

impl<T: Write, B: Backend> App<T, B> {
    pub fn new(out: T, terminal: Terminal<B>) -> Self {
        Self { out, terminal }
    }

    pub fn main_loop(&mut self) -> IoResult<()> {
        self.terminal.draw(Self::ui)?;
        self.out.flush()
    }

    pub fn event_loop(&mut self) -> IoResult<()> {
        if poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    exit(0)
                }
            }
        }
        Ok(())
    }

    fn ui(frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new("Hello World!")
                .block(Block::default().title("Greeting").borders(Borders::ALL)),
            frame.size(),
        );
    }

    fn close(&mut self) -> IoResult<()> {
        disable_raw_mode()?;
        self.out.execute(LeaveAlternateScreen)?;
        self.out.flush()?;
        Ok(())
    }
}

impl<T: Write, B: Backend> Drop for App<T, B> {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
