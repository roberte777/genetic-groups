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
