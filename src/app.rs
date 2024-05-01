use crate::{table, tui};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Buffer, Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table, Widget},
    Frame,
};
use std::io;

#[derive(Default)]
pub enum Mode {
    #[default]
    Main,
    Search,
    Exit,
}

#[derive(Default)]
pub struct App<'a> {
    pub mode: Mode,
    pub exit: bool,
    pub table: table::Table<'a>,
}

impl App<'_> {
    pub fn new(input: &str) -> App {
        let table = table::Table::from(input);
        App {
            mode: Mode::Main,
            exit: false,
            table,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_event(key.code),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) -> io::Result<()> {
        match key {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    // fn decrement(&mut self) -> Result<()> {
    //     if self.counter == 0 {
    //         bail!("counter underflow")
    //     }
    //     self.counter -= 1;
    //     Ok(())
    // }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let widths = self.table.header.iter().map(|_| Constraint::Min(0));
        let rows = self.table.rows.iter().cloned().enumerate().map(|(i, row)| {
            let row = Row::new(row);
            if i % 2 == 0 {
                row.style(Style::default().bg(Color::Black))
            } else {
                row
            }
        });
        let header = self
            .table
            .header
            .iter()
            .cloned()
            .map(Cell::new)
            .collect::<Row>()
            .style(Style::new().bold());
        Table::new(rows, widths)
            .header(header)
            .column_spacing(3)
            .block(Block::new().borders(Borders::ALL).title("Table"))
            .style(Style::new().blue())
            .render(area, buf)
    }
}
