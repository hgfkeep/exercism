use std::collections::BTreeMap;

pub struct School {
    students: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students
            .entry(grade)
            .or_insert(Vec::new())
            .push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.students.get(&grade) {
            Some(s) => {
                let mut v = s.clone();
                v.sort_unstable();
                Some(v)
            }
            _ => None,
        }
    }
}
