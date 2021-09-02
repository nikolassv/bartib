use std::cmp;
use std::fmt;
use std::str;

use nu_ansi_term::Style;

pub struct Row {
    content: Vec<String>,
    style: Option<Style>,
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
            style: None,
        }
    }

    pub fn set_color(&mut self, style: Style) {
        self.style = Some(style);
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

    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }

    fn get_all_rows(&self) -> Vec<&Row> {
        self.groups
            .iter()
            .flat_map(|g| g.rows.as_slice())
            .chain(self.rows.as_slice())
            .collect()
    }

    fn get_column_width(&self) -> Vec<usize> {
        let mut column_width: Vec<usize> = self.header.iter().map(|e| e.chars().count()).collect();

        for row in self.get_all_rows() {
            row.content
                .iter()
                .map(|cell| cell.chars().count())
                .enumerate()
                .for_each(|(i, char_count)| {
                    if let Some(old_w) = column_width.get(i) {
                        column_width[i] = cmp::max(char_count, *old_w);
                    } else {
                        column_width.push(char_count);
                    }
                });
        }

        column_width
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column_width = self.get_column_width();

        write_cells(
            f,
            &self.header,
            &column_width,
            Some(Style::new().underline()),
        )?;
        writeln!(f)?;

        for row in &self.rows {
            write_row(f, row, &column_width)?;
        }

        for group in &self.groups {
            write_group(f, group, &column_width)?;
        }

        Ok(())
    }
}

fn write_group(
    f: &mut fmt::Formatter<'_>,
    group: &Group,
    column_width: &Vec<usize>,
) -> fmt::Result {
    let empty_string = "".to_string();
    let title = group.title.as_ref().unwrap_or(&empty_string);

    writeln!(f)?;
    writeln!(f, "{}", Style::new().bold().paint(title))?;

    for row in &group.rows {
        write_row(f, row, &column_width)?;
    }

    Ok(())
}

fn write_row(f: &mut fmt::Formatter<'_>, row: &Row, column_width: &Vec<usize>) -> fmt::Result {
    write_cells(f, &row.content, &column_width, row.style)?;
    writeln!(f)?;
    Ok(())
}

fn write_cells<T: AsRef<str> + std::fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    cells: &[T],
    column_width: &[usize],
    style: Option<Style>,
) -> fmt::Result {
    let cells_with_width: Vec<(Option<&usize>, &str)> = cells
        .iter()
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

    write!(
        f,
        "{prefix}{content:<width$}{suffix} ",
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
