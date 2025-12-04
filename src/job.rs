// src/job.rs

use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Job {
    pub id: u64,
    pub created_at: Instant,
}

impl Job {
    pub fn new(id: u64) -> Self {
        Job {
            id,
            created_at: Instant::now(),
        }
    }
}
