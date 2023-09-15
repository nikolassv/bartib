use std::borrow::Cow;
use std::cmp;
use std::fmt;
use std::str;

use nu_ansi_term::Style;
use textwrap;

use crate::conf;

pub enum Wrap {
    Wrap,
    NoWrap,
}

pub struct Column {
    pub label: String,
    pub wrap: Wrap,
}

pub struct Row {
    content: Vec<String>,
    style: Option<Style>,
}

pub struct Group {
    title: Option<String>,
    rows: Vec<Row>,
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Row>,
    groups: Vec<Group>,
}

impl Row {
    pub fn new(content: Vec<String>) -> Self {
        Self {
            content,
            style: None,
        }
    }

    pub fn set_color(&mut self, style: Style) {
        self.style = Some(style);
    }
}

impl Group {
    pub fn new(title: Option<String>, rows: Vec<Row>) -> Self {
        Self { title, rows }
    }
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
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

    /* Calculates the widths for the columns in a table

       If the width of the longest line in the table exceeds the maximum width for the output
       all the wrapable columns shrink to an acceptable size.
    */
    fn get_column_width(&self, max_width: usize) -> Vec<usize> {
        let mut max_column_width = self.get_max_column_width();

        let width: usize = max_column_width.iter().sum();
        let columns_wrap: Vec<&Wrap> = self.columns.iter().map(|c| &c.wrap).collect();
        let mut number_of_wrappable_columns: usize = columns_wrap
            .iter()
            .filter(|w| matches!(w, Wrap::Wrap))
            .count();

        if width <= max_width || number_of_wrappable_columns == 0 {
            // we do not need to or can not wrap
            return max_column_width;
        }

        // the total width of the columns that we may not wrap
        let unwrapable_width: usize = max_column_width
            .iter()
            .zip(columns_wrap.iter())
            .filter(|(_, wrap)| matches!(wrap, Wrap::NoWrap))
            .map(|(width, _)| width)
            .sum();

        if unwrapable_width > max_width {
            // In this case we can not get any decent layout with wrapping. We rather do not wrap at all
            return max_column_width;
        }

        // we start with a width of 0 for all the wrapable columns
        let mut column_width: Vec<usize> = max_column_width
            .iter()
            .zip(columns_wrap.iter())
            .map(|(width, wrap)| {
                if matches!(wrap, Wrap::NoWrap) {
                    *width
                } else {
                    0
                }
            })
            .collect();

        // then we distribute the available width to the wrappable columns
        let mut available_width_for_wrappable_columns = max_width - unwrapable_width;

        while available_width_for_wrappable_columns > 0 && number_of_wrappable_columns > 0 {
            // the maximum additional width we give each column in this round
            let additional_width_for_each = cmp::max(
                1,
                available_width_for_wrappable_columns / number_of_wrappable_columns,
            );
            let width_data = column_width
                .iter_mut()
                .zip(max_column_width.iter_mut())
                .zip(columns_wrap.iter());

            for ((width, max_width), wrap) in width_data {
                if available_width_for_wrappable_columns > 0
                    && matches!(wrap, Wrap::Wrap)
                    && width < max_width
                {
                    if max_width > &mut width.saturating_add(additional_width_for_each) {
                        // While the maximum width for this column will not be reached, we add all
                        // the additional width
                        available_width_for_wrappable_columns -= additional_width_for_each;
                        *width = width.saturating_add(additional_width_for_each);
                    } else {
                        // The column does not need all the additional width. We give it only the
                        // additional width it needs
                        available_width_for_wrappable_columns -= *max_width - *width;
                        *width = *max_width;

                        // this column won't need any more width
                        number_of_wrappable_columns -= 1;
                    }
                }
            }
        }

        column_width
    }

    fn get_max_column_width(&self) -> Vec<usize> {
        let mut max_column_width: Vec<usize> = self
            .columns
            .iter()
            .map(|c| c.label.chars().count())
            .collect();

        for row in self.get_all_rows() {
            row.content
                .iter()
                .map(|cell| cell.chars().count())
                .enumerate()
                .for_each(|(i, char_count)| {
                    if let Some(old_w) = max_column_width.get(i) {
                        max_column_width[i] = cmp::max(char_count, *old_w);
                    } else {
                        max_column_width.push(char_count);
                    }
                });
        }
        max_column_width
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terminal_width = term_size::dimensions_stdout().map_or(conf::DEFAULT_WIDTH, |d| d.0);

        let column_width = self.get_column_width(terminal_width - self.columns.len());

        let labels: Vec<&String> = self.columns.iter().map(|c| &c.label).collect();

        write_cells(f, &labels, &column_width, Some(Style::new().underline()))?;
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

fn write_group(f: &mut fmt::Formatter<'_>, group: &Group, column_width: &[usize]) -> fmt::Result {
    let empty_string = String::new();
    let title = group.title.as_ref().unwrap_or(&empty_string);

    writeln!(f)?;
    writeln!(f, "{}", Style::new().bold().paint(title))?;

    for row in &group.rows {
        write_row(f, row, column_width)?;
    }

    Ok(())
}

fn write_row(f: &mut fmt::Formatter<'_>, row: &Row, column_width: &[usize]) -> fmt::Result {
    write_cells(f, &row.content, column_width, row.style)?;
    writeln!(f)?;
    Ok(())
}

fn write_cells<T: AsRef<str> + std::fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    cells: &[T],
    column_width: &[usize],
    style: Option<Style>,
) -> fmt::Result {
    let wrapped_cells: Vec<Vec<Cow<str>>> = cells
        .iter()
        .enumerate()
        .map(|(i, c)| match column_width.get(i) {
            Some(s) => textwrap::wrap(c.as_ref(), textwrap::Options::new(*s)),
            None => {
                vec![Cow::from(c.as_ref())]
            }
        })
        .collect();

    let most_lines: usize = wrapped_cells
        .iter()
        .map(std::vec::Vec::len)
        .max()
        .unwrap_or(1);

    for line in 0..most_lines {
        for (width, wrapped_cell) in column_width.iter().zip(wrapped_cells.iter()) {
            match wrapped_cell.get(line) {
                Some(c) => write_with_width_and_style(f, c, width, style)?,
                None => write!(f, "{} ", "\u{a0}".repeat(*width))?, // pad with non breaking space
            }
        }

        let is_last_line = line + 1 < most_lines;
        if is_last_line {
            writeln!(f)?;
        }
    }

    Ok(())
}

fn write_with_width_and_style(
    f: &mut fmt::Formatter<'_>,
    content: &str,
    width: &usize,
    opt_style: Option<Style>,
) -> fmt::Result {
    let style_prefix = opt_style.map_or(String::new(), |style| style.prefix().to_string());
    let style_suffix = opt_style.map_or(String::new(), |style| style.suffix().to_string());

    // cells are filled with non-breaking white space. Contrary to normal spaces non-breaking white
    // space will be styled (e.g. underlined)
    write!(f, "{style_prefix}{content:\u{a0}<width$}{style_suffix} ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_column_width_without_wrapping() {
        let mut t = Table::new(get_columns());
        let row1 = Row::new(vec!["abc".to_string(), "defg".to_string()]);
        let row2 = Row::new(vec!["a".to_string(), "b".to_string(), "cdef".to_string()]);

        t.add_row(row1);
        t.add_row(row2);

        let column_width = t.get_column_width(100);

        assert_eq!(column_width[0], 3);
        assert_eq!(column_width[1], 4);
        assert_eq!(column_width[2], 4);
    }

    #[test]
    fn get_column_width_with_wrapping() {
        let mut t = Table::new(vec![
            Column {
                label: "a".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "b".to_string(),
                wrap: Wrap::Wrap,
            },
            Column {
                label: "c".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "d".to_string(),
                wrap: Wrap::Wrap,
            },
            Column {
                label: "e".to_string(),
                wrap: Wrap::Wrap,
            },
            Column {
                label: "e".to_string(),
                wrap: Wrap::Wrap,
            },
        ]);
        let row1 = Row::new(vec![
            "abcdefg".to_string(),         // 7
            "abcdefghijkl".to_string(),    // 12 -> muss gewrapt werden
            "abcde".to_string(),           // 5
            "abc".to_string(),             // 3 -> muss nicht gewrapt werden
            "abcdefghijklmno".to_string(), // 15 -> muss gewrapt werden
            "abcdefg".to_string(),         // 7 -> muss nicht gewrapt werden
        ]);

        t.add_row(row1);

        let column_width = t.get_column_width(7 + 5 + 25);

        assert_eq!(column_width[0], 7);
        assert_eq!(column_width[1], 8);
        assert_eq!(column_width[2], 5);
        assert_eq!(column_width[3], 3);
        assert_eq!(column_width[4], 7);
        assert_eq!(column_width[5], 7);
    }

    #[test]
    fn get_column_width_with_wrapping_not_possible() {
        let mut t = Table::new(vec![
            Column {
                label: "a".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "b".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "c".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "d".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "e".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "e".to_string(),
                wrap: Wrap::NoWrap,
            },
        ]);
        let row1 = Row::new(vec![
            "abcdefg".to_string(),         // 7
            "abcdefghijkl".to_string(),    // 12
            "abcde".to_string(),           // 5
            "abc".to_string(),             // 3
            "abcdefghijklmno".to_string(), // 15
            "abcdefg".to_string(),         // 7
        ]);

        t.add_row(row1);

        let column_width = t.get_column_width(10);

        assert_eq!(column_width[0], 7);
        assert_eq!(column_width[1], 12);
        assert_eq!(column_width[2], 5);
        assert_eq!(column_width[3], 3);
        assert_eq!(column_width[4], 15);
        assert_eq!(column_width[5], 7);
    }

    #[test]
    fn get_column_width_with_wrapping_not_enough_wrappable_space() {
        let mut t = Table::new(vec![
            Column {
                label: "a".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "b".to_string(),
                wrap: Wrap::Wrap,
            },
            Column {
                label: "c".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "d".to_string(),
                wrap: Wrap::Wrap,
            },
            Column {
                label: "e".to_string(),
                wrap: Wrap::Wrap,
            },
            Column {
                label: "e".to_string(),
                wrap: Wrap::Wrap,
            },
        ]);
        let row1 = Row::new(vec![
            "abcdefg".to_string(),         // 7
            "abcdefghijkl".to_string(),    // 12
            "abcde".to_string(),           // 5
            "abc".to_string(),             // 3
            "abcdefghijklmno".to_string(), // 15
            "abcdefg".to_string(),         // 7
        ]);

        t.add_row(row1);

        let column_width = t.get_column_width(10);

        assert_eq!(column_width[0], 7);
        assert_eq!(column_width[1], 12);
        assert_eq!(column_width[2], 5);
        assert_eq!(column_width[3], 3);
        assert_eq!(column_width[4], 15);
        assert_eq!(column_width[5], 7);
    }

    #[test]
    fn display() {
        let mut t = Table::new(get_columns());
        let row1 = Row::new(vec!["abc".to_string(), "defg".to_string()]);
        let row2 = Row::new(vec!["a".to_string(), "b".to_string(), "cdef".to_string()]);

        t.add_row(row1);
        t.add_row(row2);

        assert_eq!(
            format!("{t}"),
            "\u{1b}[4ma\u{a0}\u{a0}\u{1b}[0m \u{1b}[4mb\u{a0}\u{a0}\u{a0}\u{1b}[0m \u{1b}[4mc\u{a0}\u{a0}\u{a0}\u{1b}[0m \nabc defg \na\u{a0}\u{a0} b\u{a0}\u{a0}\u{a0} cdef \n"
        );
    }

    fn get_columns() -> Vec<Column> {
        vec![
            Column {
                label: "a".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "b".to_string(),
                wrap: Wrap::NoWrap,
            },
            Column {
                label: "c".to_string(),
                wrap: Wrap::NoWrap,
            },
        ]
    }
}
