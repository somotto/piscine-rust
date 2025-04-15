#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

// Define Link as an Option containing a Box of Worker
// This allows for recursive structure with null termination
pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Add a worker at the start of the list
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        
        self.grade = Some(Box::new(new_worker));
    }

    // Remove the last worker that was placed (first in the list)
    // If there's a worker to remove
    // Update the head to point to the next worker
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            self.grade = worker.next;
            worker.name
        })
    }

    // Return information about the last added worker (first in the list)
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| {
            (worker.name.clone(), worker.role.clone())
        })
    }
}