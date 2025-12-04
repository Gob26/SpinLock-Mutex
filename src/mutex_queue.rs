// src/mutex_queue.rs


use std::collections::VecDeque;
use std::sync::Mutex;

pub struct Job {
    pub id: u64
}

impl Job {
    pub fn new(id: u64) -> Self {
        Job { id }
    }
}

pub struct MutexJobQueue {
    queue: Mutex<VecDeque<Job>>,
}

impl MutexJobQueue {
    pub fn new() -> Self {
        MutexJobQueue {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push(&self, job: Job) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(job);
    }

    pub fn pop(&self) -> Option<Job> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()

    }
}
