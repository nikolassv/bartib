use std::cmp;
use std::fmt;
use std::fmt::Formatter;
use std::str;

use ansi_term::{Colour, Style};

pub struct Row {
    content: Vec<String>,
    color: Option<Colour>,
}

pub struct Group {
    title: Option<String>,
    rows: Vec<Row>,
}

pub struct Table {
    header: Vec<String>,
    rows: Vec<Row>,
    groups: Vec<Group>,
}

impl Row {
    pub fn new(content: Vec<String>) -> Row {
        Row {
            content,
            color: None,
        }
    }

    pub fn set_color(&mut self, color: Colour) {
        self.color = Some(color);
    }
}

impl Group {
    pub fn new(title: Option<String>, rows: Vec<Row>) -> Group {
        Group { title, rows }
    }
}

impl Table {
    pub fn new(header: Vec<String>) -> Table {
        Table {
            header,
            groups: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    fn get_column_width(&self) -> Vec<usize> {
        let mut column_width: Vec<usize> = self.header.iter().map(|e| e.chars().count()).collect();

        let mut i: usize;

        for row in &self.rows {
            i = 0;

            while let Some(w) = row.content.get(i) {
                if let Some(old_w) = column_width.get(i) {
                    column_width[i] = cmp::max(w.chars().count(), *old_w);
                } else {
                    column_width.push(w.chars().count());
                }

                i += 1;
            }
        }

        column_width
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column_width = self.get_column_width();

        write_cells(f, &self.header, &column_width, Some(Style::new().underline()))?;
        writeln!(f)?;

        for row in &self.rows {
            let style = row.color.map(|color| Style::new().fg(color));
            write_cells(f, &row.content, &column_width, style)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

fn write_cells<T: AsRef<str> + std::fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    cells: &[T],
    column_width: &[usize],
    style: Option<Style>,
) -> fmt::Result {
    let cells_with_width : Vec<(Option<&usize>, &str)> = cells.iter()
        .map(|cell| cell.as_ref())
        .enumerate()
        .map(|(i, cell)| (column_width.get(i), cell))
        .collect();

    for (width, cell) in cells_with_width {
        write_with_width_and_style(f, cell, width, style)?;
    }

    Ok(())
}

fn write_with_width_and_style(
    f: &mut fmt::Formatter<'_>,
    content: &str,
    opt_width: Option<&usize>,
    opt_style: Option<Style>,
) -> fmt::Result {
    let content_length = content.chars().count();
    let style_prefix = opt_style.map_or("".to_string(), |style| style.prefix().to_string());
    let style_suffix = opt_style.map_or("".to_string(), |style| style.suffix().to_string());
    let width = opt_width.unwrap_or(&content_length);

    write!(f, "{prefix}{content:<width$}{suffix} ",
           prefix = style_prefix,
           content = content,
           width = width,
           suffix = style_suffix
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_column_width() {
        let mut t = Table::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let row1 = Row::new(vec!["abc".to_string(), "defg".to_string()]);
        let row2 = Row::new(vec!["a".to_string(), "b".to_string(), "cdef".to_string()]);

        t.add_row(row1);
        t.add_row(row2);

        let column_width = t.get_column_width();

        assert_eq!(column_width[0], 3);
        assert_eq!(column_width[1], 4);
        assert_eq!(column_width[2], 4);
    }

    #[test]
    fn display() {
        let mut t = Table::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let row1 = Row::new(vec!["abc".to_string(), "defg".to_string()]);
        let row2 = Row::new(vec!["a".to_string(), "b".to_string(), "cdef".to_string()]);

        t.add_row(row1);
        t.add_row(row2);

        assert_eq!(
            format!("{}", t),
            "\u{1b}[4ma  \u{1b}[0m \u{1b}[4mb   \u{1b}[0m \u{1b}[4mc   \u{1b}[0m \nabc defg \na   b    cdef \n"
        );
    }
}
