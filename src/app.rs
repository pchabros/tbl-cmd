use crate::tui;
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
    pub header: Vec<&'a str>,
    pub rows: Vec<Vec<&'a str>>,
}

impl App<'_> {
    pub fn new(input: &str) -> App {
        let mut lines: Vec<&str> = input.split('\n').collect();
        let header = lines
            .remove(0)
            .split("  ")
            .map(|cell| cell.trim())
            .collect();
        let rows = lines
            .iter()
            .map(|line| line.split("  ").map(|cell| cell.trim()).collect())
            .collect();

        App {
            mode: Mode::Main,
            exit: false,
            header,
            rows,
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
        // let n_col: u32 = self
        //     .header
        //     .len()
        //     .try_into()
        //     .expect("number of columns should not overflow u32");
        // let widths = self.header.iter().map(|_| Constraint::Ratio(1, n_col));
        let widths = self.header.iter().map(|_| Constraint::Length(100));
        let rows = self.rows.iter().map(|row| Row::new(row.clone()));
        let header = self
            .header
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                Cell::new(*cell).style(Style::default().bg(if i % 2 == 0 {
                    Color::Yellow
                } else {
                    Color::LightRed
                }))
            })
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
