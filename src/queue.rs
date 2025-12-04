use std::collections::VecDeque;
use crate::spinlock::SpinLock;
use crate::job::Job;
//TO DO


pub struct JobQueue {
    inner: SpinLock<VecDeque<Job>>,
    //TO DO
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue { 
            inner: SpinLock::new(VecDeque::new()),
    }
}

    pub fn push(&self, job: Job) {
        let mut guard = self.inner.lock();

        guard.push_back(job);
}

    pub fn pop(&self) -> Option<Job> {
        let mut guard = self.inner.lock();
    
        guard.pop_front()
    }
}
