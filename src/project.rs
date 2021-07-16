use std::fmt;

#[derive(Debug)]
pub struct Project(pub String);

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
