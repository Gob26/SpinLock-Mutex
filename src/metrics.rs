// src/metrics.rs
//
use std::sync::atomic::{AtomicU64, Ordering};

// Структура для сбора статист
pub struct LockMetrics {
    pub lock_count: AtomicU64,
    pub spin_count: AtomicU64,
    pub wait_ns_total: AtomicU64,
    pub contentions: AtomicU64,
}

impl LockMetrics {
    pub fn new() -> Self { 
        LockMetrics{
            lock_count: AtomicU64::new(0),
            spin_count: AtomicU64::new(0),
            wait_ns_total: AtomicU64::new(0),
            contentions: AtomicU64::new(0),
    }
}

    pub fn record(&self, spins: u64, wait_ns: u64, contended: bool) {
        self.lock_count.fetch_add(1, Ordering::Relaxed); // Получить и добавить
        self.spin_count.fetch_add(spins, Ordering::Relaxed);
        self.wait_ns_total.fetch_add(wait_ns, Ordering::Relaxed);

        if contended {
            self.contentions.fetch_add(1, Ordering::Relaxed);
        }
    }
}

