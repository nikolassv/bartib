use std::cmp;
use std::fmt;
use std::str;

pub struct Row {
	content: Vec<String>
}

pub struct Table<'a> {
	header: Vec<&'a str>,
	rows: Vec<Row>
}

impl Row {
	pub fn new(content : Vec<String>) -> Row {
		Row{content}
	}
}

impl<'a> Table<'a> {
	pub fn new(header : Vec<&'a str>) -> Table<'a> {
		Table {
			header,
			rows: Vec::new()
		}
	}
	
	pub fn add_row(&mut self, row : Row) {
		self.rows.push(row);
	}
	
	fn get_column_width(&self) -> Vec<usize> {
		let mut column_width : Vec<usize> = self.header.iter().map(|e| e.chars().count()).collect();
		
		let mut i : usize;
		
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
	
	fn write_cells<T : AsRef<str> + std::fmt::Display>(column_width : &[usize], cells : &[T], f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut i = 0;
		
		while let Some(cell) = cells.get(i) {
			if let Some(width) = column_width.get(i) {
				write!(f, "{:<width$} ", cell, width = width)?;
			} else {
				write!(f, "{} ", cell)?;
			}
			
			i += 1;
		}
		
		Ok(())
	}
	
	fn write_horizontal_line(column_width : &[usize], f : &mut fmt::Formatter<'_>) -> fmt::Result {
		for width in column_width {
			write!(f, "{} ", "-".repeat(*width))?;
		}
		
		Ok(())
	}
}

impl <'a> fmt::Display for Table<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let column_width = self.get_column_width();
		
		Table::write_cells(&column_width, &self.header, f)?;
		writeln!(f)?;
		Table::write_horizontal_line(&column_width, f)?;
		writeln!(f)?;
		
		for row in &self.rows {
			Table::write_cells(&column_width, &row.content, f)?;
			writeln!(f)?;
		}
		
		Ok(())
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn get_column_width() {
		let mut t = Table::new(vec!["a", "b", "c"]);
		t.add_row(Row::new(vec!["abc".to_string(), "defg".to_string()]));
		t.add_row(Row::new(vec!["a".to_string(), "b".to_string(), "cdef".to_string()]));

		let column_width = t.get_column_width();
				
		assert_eq!(column_width[0], 3);
		assert_eq!(column_width[1], 4);
		assert_eq!(column_width[2], 4);
	}
	
	#[test]
	fn display() {
		let mut t = Table::new(vec!["a", "b", "c"]);
		t.add_row(Row::new(vec!["abc".to_string(), "defg".to_string()]));
		t.add_row(Row::new(vec!["a".to_string(), "b".to_string(), "cdef".to_string()]));

		assert_eq!(format!("{}", t), "a   b    c    \n--- ---- ---- \nabc defg \na   b    cdef \n");
	}
}