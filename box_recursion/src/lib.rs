#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    // Initialize WorkEnvironment with no workers
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Add a worker at the start of the list
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(), // Take current head and set as next
        };
        self.grade = Some(Box::new(new_worker));
    }

    // Remove the worker at the front of the list and return the name
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            self.grade = worker.next; // Advance head
            worker.name
        })
    }

    // Return the name and role of the last added worker
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}
