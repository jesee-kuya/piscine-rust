#[derive(Debug, PartialEq)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, PartialEq)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };

        self.grade = Some(Box::new(worker))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            self.grade = worker.next;
            worker.name
        })
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = WorkEnvironment::new();
        assert_eq!(list, WorkEnvironment { grade: None });
        list.add_worker(String::from("CEO"), String::from("Marie"));
        assert_eq!(list, WorkEnvironment { grade: Some( Box::new(Worker{
            role: "CEO".to_string(),
            name: "Marie".to_string(),
            next: None,
        }))});
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.add_worker(String::from("Normal Worker"), String::from("Ana"));
        list.add_worker(String::from("Normal Worker"), String::from("Alice"));
        list.remove_worker();
        list.remove_worker();
        list.remove_worker();
        assert_eq!(list, WorkEnvironment { grade: Some( Box::new(Worker{
            role: "CEO".to_string(),
            name: "Marie".to_string(),
            next: None,
        }))});
    }
}