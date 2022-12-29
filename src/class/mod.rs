use std::fmt::{Display, Error, Formatter};

#[derive(Clone)]
pub struct Student {
    pub name: String,
    pub blocked_grouping: Vec<Student>,
}

impl Student {
    pub fn new(name: String, blocked_grouping: Option<Vec<Student>>) -> Student {
        Student {
            name,
            blocked_grouping: blocked_grouping.unwrap_or(Vec::new()),
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.name)
    }
}
