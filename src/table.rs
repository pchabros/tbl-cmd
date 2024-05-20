use regex::Regex;

#[derive(Default)]
pub struct Row<'a> {
    pub cells: Vec<&'a str>,
}

impl<'a> Row<'a> {
    fn new(line: &'a str, indexes: &[usize]) -> Self {
        let cells = indexes
            .iter()
            .map(|index| {
                let re = Regex::new(r"^(\S ?)+").unwrap();
                let (char_index, _) = line.char_indices().nth(*index).unwrap_or((0, ' '));
                let line_left = &line[char_index..];
                let cell_match = re.find(line_left);
                match cell_match {
                    Some(pattern) => pattern.as_str().trim(),
                    None => "",
                }
            })
            .collect();
        Self { cells }
    }
    fn contains(&self, pattern: &str) -> Option<&Self> {
        if self.cells.iter().any(|cell| cell.contains(pattern)) {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Table<'a> {
    pub header: Row<'a>,
    pub rows: Vec<Row<'a>>,
    header_line: &'a str,
    row_lines: Vec<&'a str>,
    indexes: Vec<usize>,
}

impl<'a> Table<'a> {
    fn set_header_and_row_lines(mut self, s: &'a str) -> Self {
        let mut lines: Vec<&str> = s.lines().filter(|line| !line.is_empty()).collect();
        self.header_line = lines.remove(0);
        self.row_lines = lines;
        self
    }
    fn set_columns_indexes(mut self) -> Self {
        let re = Regex::new(r"(\S+ ?)+").unwrap();
        self.indexes = re.find_iter(self.header_line).map(|m| m.start()).collect();
        self
    }
    fn parse_header_cells(mut self) -> Self {
        self.header = Row::new(self.header_line, &self.indexes);
        self
    }
    fn parse_row_cells(mut self) -> Self {
        self.rows = self
            .row_lines
            .iter()
            .map(|line| Row::new(line, &self.indexes))
            .collect();
        self
    }
    pub fn filtered_rows(&self, pattern: &str) -> Vec<&Row> {
        self.rows
            .iter()
            .flat_map(|row| row.contains(pattern))
            .collect()
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

        assert_eq!(table.header.cells, vec!["header 1", "header 2", "header 3"]);
        assert_eq!(table.rows[0].cells, vec!["a", "bbbbbbbbb…", "ccc"]);
        assert_eq!(table.rows[1].cells, vec!["dddddddddd", "", "f"]);
    }
}
