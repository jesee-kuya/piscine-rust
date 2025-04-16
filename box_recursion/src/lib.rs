#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Worker>),
}

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: Link::Empty }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: std::mem::replace(&mut self.grade, Link::Empty),
        };
        self.grade = Link::More(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match std::mem::replace(&mut self.grade, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_worker) => {
                let Worker { name, next, .. } = *boxed_worker;
                self.grade = next;
                Some(name)
            }
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            Link::Empty => None,
            Link::More(worker) => Some((worker.name.clone(), worker.role.clone())),
        }
    }
}
