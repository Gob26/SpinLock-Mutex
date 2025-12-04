// tests/spinlock_test.rs
//
use spinmetrics::SpinLock;
use std::sync::Arc;
use std::thread;
use std::sync::atomic::Ordering;

#[test]
fn integration_contention_test() {
    const THREADS: usize = 4;
    const INCREMENTS_PER_THREAD: usize = 10_000;
    const EXPECTED_TOTAL: i32 = (THREADS * INCREMENTS_PER_THREAD) as i32;
    let lock = Arc::new(SpinLock::new(0_i32));
    let mut handles = vec![];

    println!("Запуск {} потоков, каждый делает {} инкрементов...", 
             THREADS, INCREMENTS_PER_THREAD);

    for _ in 0..THREADS {
        let lock_clone = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _ in 0..INCREMENTS_PER_THREAD {
                let mut data = lock_clone.lock();
                *data += 1;
            }
        }); 
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

// === НОВЫЙ КОД: ВЫВОД МЕТРИК ===
    let metrics = &lock.metrics;

    let total_locks = metrics.lock_count.load(Ordering::Relaxed);
    let total_spins = metrics.spin_count.load(Ordering::Relaxed);
    let total_wait_ns = metrics.wait_ns_total.load(Ordering::Relaxed);
    let total_contentions = metrics.contentions.load(Ordering::Relaxed);

    println!("--- МЕТРИКИ SPINLOCK ---");
    println!("Всего захватов (Lock Count): {}", total_locks);
    println!("Всего спинов (Spin Count): {}", total_spins);
    println!("Всего ожидания (Wait NS): {} нс", total_wait_ns);
    println!("Всего оспариваний (Contentions): {}", total_contentions);
    
    // Вычисляем средние спины на оспаривание
    let avg_spins_per_contention = if total_contentions > 0 {
        total_spins as f64 / total_contentions as f64
    } else {
        0.0
    };
    
    println!("Средние спины на оспаривание: {:.2}", avg_spins_per_contention);
    // =============================

    // Проверяем финальный результат.
    let final_value = *lock.lock(); 

    println!("Финальное значение: {}", final_value);
    println!("Ожидаемое значение: {}", EXPECTED_TOTAL);

    assert_eq!(final_value, EXPECTED_TOTAL, 
               "Data race обнаружен! Ожидалось {}, получено {}", 
               EXPECTED_TOTAL, final_value);
}
