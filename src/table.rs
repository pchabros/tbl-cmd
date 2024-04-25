use regex::Regex;
use std::str::FromStr;

pub struct Table<'a> {
    pub header: Vec<String>,
    pub rows: Vec<Vec<&'a str>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTableError;

impl FromStr for Table<'_> {
    type Err = ParseTableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<&str> = s.lines().filter(|line| !line.is_empty()).collect();
        let header_line = lines.remove(0);

        let re = Regex::new(r"(\w+ \w)+").unwrap();
        let indexes: Vec<usize> = re.find_iter(header_line).map(|m| m.start()).collect();

        let mut header = Vec::new();
        for index in indexes {
            let header_cell = match re.find(header_line.split_at(index).1) {
                Some(pattern) => pattern.as_str(),
                None => "",
            };
            header.push(header_cell.to_string())
        }

        let table = Table {
            header,
            rows: Vec::new(),
        };
        Ok(table)
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use std::str::FromStr;

    #[test]
    fn parse_table_from_string() {
        let string = "\n\
            header 1    header 2    header 3\n\
            a           bbbbbbbbbb  ccc     \n\
            dddddddddd              f       \n\
        ";
        let table = Table::from_str(string).unwrap();

        assert_eq!(table.header, vec!["header 1", "header 2", "header 3"]);
        // assert_eq!(
        //     table.rows,
        //     vec![vec!["a", "bbbbbbbbbb", "ccc"], vec!["dddddddddd", "", "f"]]
        // );
    }
}
