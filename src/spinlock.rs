use std::thread;
// src/spinlock.rs
use std::time::Instant;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;
use std::hint::spin_loop;
use crate::metrics::LockMetrics;


const SPIN_LIMIT_BEFORE_YIELD: u64 = 10;

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
    //TODO:
    pub metrics: LockMetrics,
}

pub struct SpinGuard<'a, T: 'a> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
// Тут мы даем гарантируем безопасность 
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        SpinLock { 
            locked: AtomicBool::new(false), 
            data: UnsafeCell::new(data),
            metrics: LockMetrics::new(),
        }
    }


    pub fn lock(&self) -> SpinGuard<'_, T> {
        let start = Instant::now();
        let mut spins: u64 = 0;
        let mut retries: u64 = 0; // Счетчик попыток


//        while self.locked.compare_exchange( // Обмен и сравнение CAS
  //          false,
    //        true,
      //      Ordering::Acquire,
        //    Ordering::Relaxed,
//        ).is_err() {
  //          spins += 1;
    //        spin_loop();
      //  }
            while self.locked.swap(true, Ordering::Acquire) {
                spins += 1;
                retries += 1;
                
                if retries > SPIN_LIMIT_BEFORE_YIELD {
                    thread::yield_now();
                    retries = 0;
            }

            spin_loop();

        } 

        let elapsed = start.elapsed();
        let wait_ns = elapsed.as_nanos() as u64;

        let contended = spins > 0;
        self.metrics.record(spins, wait_ns, contended);

        SpinGuard {lock: self}
    }
}

impl<'a, T> Deref for SpinGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.lock.data.get()}
        }
    }

impl<'a, T> DerefMut for SpinGuard<'a, T> {
        fn deref_mut( &mut self) -> &mut Self::Target {
            unsafe { &mut *self.lock.data.get()}
        }
    }
    
impl<'a, T> Drop for SpinGuard<'a, T> {
        fn drop(&mut self) {
            self.lock.locked.store(false, Ordering::Release);
        }
    }


