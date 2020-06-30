pub struct School {
    classes: std::collections::BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            classes: std::collections::BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.classes
            .entry(grade)
            .and_modify(|e| e.push(student.to_string()))
            .or_insert(vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.classes.keys().map(|k| *k).collect()
    }
    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.classes
            .get(&grade)
            .cloned()
            .and_then(|mut students_for_grade| {
                students_for_grade.sort();
                Some(students_for_grade)
            })
    }
}
