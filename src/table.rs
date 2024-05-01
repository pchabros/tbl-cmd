use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Table<'a> {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
    header_line: &'a str,
    row_lines: Vec<&'a str>,
    indexes: Vec<usize>,
}

impl<'a> Table<'a> {
    fn set_header_and_row_lines(mut self, s: &'a str) -> Table<'a> {
        let mut lines: Vec<&str> = s.lines().filter(|line| !line.is_empty()).collect();
        self.header_line = lines.remove(0);
        self.row_lines = lines;
        self
    }
    fn set_columns_indexes(mut self) -> Table<'a> {
        let re = Regex::new(r"(\S+ ?)+").unwrap();
        self.indexes = re.find_iter(self.header_line).map(|m| m.start()).collect();
        self
    }
    fn parse_cells(&self, line: &str) -> Vec<String> {
        self.indexes
            .iter()
            .map(|index| {
                let re = Regex::new(r"^(\S ?)+").unwrap();
                let line_left = line.graphemes(true).skip(*index).collect::<String>();
                let cell_match = re.find(&line_left);
                let cell = match cell_match {
                    Some(pattern) => pattern.as_str().trim(),
                    None => "",
                };
                cell.to_string()
            })
            .collect()
    }
    fn parse_header_cells(mut self) -> Table<'a> {
        self.header = self.parse_cells(self.header_line);
        self
    }
    fn parse_row_cells(mut self) -> Table<'a> {
        self.rows = self
            .row_lines
            .iter()
            .map(|line| self.parse_cells(line))
            .collect();
        self
    }
}

impl<'a> From<&'a str> for Table<'a> {
    fn from(s: &'a str) -> Table<'a> {
        Table::default()
            .set_header_and_row_lines(s)
            .set_columns_indexes()
            .parse_header_cells()
            .parse_row_cells()
    }
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn parse_table_from_string() {
        let string = "\n\
            header 1    header 2    header 3\n\
            a           bbbbbbbbb…  ccc     \n\
            dddddddddd              f       \n\
        ";
        let table = Table::from(string);

        assert_eq!(table.header, vec!["header 1", "header 2", "header 3"]);
        assert_eq!(
            table.rows,
            vec![vec!["a", "bbbbbbbbb…", "ccc"], vec!["dddddddddd", "", "f"]]
        );
    }
}
