use crate::{table, tui};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use std::io;

#[derive(Default, PartialEq)]
pub enum Mode {
    #[default]
    Main,
    Search,
    Exit,
}

#[derive(Default)]
pub struct App<'a> {
    pub mode: Mode,
    pub table: table::Table<'a>,
    pub search: String,
}

impl App<'_> {
    pub fn new(input: &str) -> App {
        let table = table::Table::from(input);
        App {
            mode: Mode::Main,
            table,
            search: String::new(),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_event(key.code),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) -> io::Result<()> {
        match self.mode {
            Mode::Main => match key {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('/') => self.search_mode(),
                _ => {}
            },
            Mode::Search => match key {
                KeyCode::Char(_) | KeyCode::Backspace => self.update_search(key),
                KeyCode::Esc => self.main_mode(),
                _ => {}
            },
            Mode::Exit => {}
        }
        Ok(())
    }

    fn main_mode(&mut self) {
        self.mode = Mode::Main;
    }

    fn search_mode(&mut self) {
        self.mode = Mode::Search;
    }

    fn update_search(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(letter) => {
                self.search.push(letter);
            }
            KeyCode::Backspace => {
                self.search.pop();
            }
            _ => {}
        };
    }

    fn exit(&mut self) {
        self.mode = Mode::Exit;
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while self.mode != Mode::Exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }
}
