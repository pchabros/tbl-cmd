use std::rc::Rc;

use ratatui::{
    prelude::{Buffer, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Cell, Paragraph, Row, Table, Widget},
};

use crate::app::{App, Mode};

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = self.layout(area);
        self.render_search(areas[0], buf);
        self.render_table(areas[1], buf);
    }
}

impl App<'_> {
    fn layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10)])
            .split(area)
    }
    fn render_table(&self, area: Rect, buf: &mut Buffer) {
        let widths = self.table.header.cells.iter().map(|_| Constraint::Min(0));
        let filtered_rows = self.table.filtered_rows(&self.search);
        let rows = filtered_rows.iter().enumerate().map(|(i, row)| {
            let row = Row::new(row.cells.iter().map(|cell| Cell::new(*cell)));
            if i % 2 == 0 {
                row.style(Style::default().bg(Color::Black))
            } else {
                row
            }
        });
        let header = Row::new(self.table.header.cells.iter().map(|cell| Cell::new(*cell)))
            .style(Style::new().bold());
        Table::new(rows, widths)
            .header(header)
            .block(Block::bordered().highlight_if(self.mode == Mode::Main))
            .render(area, buf)
    }
    fn render_search(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.search.clone())
            .block(
                Block::bordered()
                    .title("Search")
                    .highlight_if(self.mode == Mode::Search),
            )
            .render(area, buf);
    }
}

trait Highlight {
    fn highlight_if(self, active: bool) -> Self;
}

impl Highlight for Block<'_> {
    fn highlight_if(self, active: bool) -> Self {
        self.border_style(Style::default().fg(if active { Color::Blue } else { Color::White }))
    }
}
