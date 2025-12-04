// tests/mutex_queue_test.rs

use spinmetrics::mutex_queue::{MutexJobQueue, Job}; // Используем новую очередь
use std::sync::Arc;
use std::thread;

// Параметры симуляции
const NUM_PRODUCERS: usize = 4;
const NUM_CONSUMERS: usize = 4;
const JOBS_PER_PRODUCER: usize = 10000;
const TOTAL_JOBS: usize = NUM_PRODUCERS * JOBS_PER_PRODUCER;

#[test]
fn mutex_producer_consumer_integration_test() {
    println!("Запуск симуляции Producer/Consumer с Mutex:");
    println!("Производителей: {}", NUM_PRODUCERS);
    println!("Потребителей: {}", NUM_CONSUMERS);
    println!("Всего задач: {}", TOTAL_JOBS);

    // 1. Инициализация очереди и счетчика
    let queue = Arc::new(MutexJobQueue::new());
    // std::sync::atomic::AtomicUsize подходит для подсчета задач
    let jobs_processed = Arc::new(std::sync::atomic::AtomicUsize::new(0)); 
    let mut handles = Vec::new();

    // --- 2. Запуск Производителей (Producers) ---
    for i in 0..NUM_PRODUCERS {
        let queue_clone = Arc::clone(&queue);
        let producer_handle = thread::spawn(move || {
            for j in 0..JOBS_PER_PRODUCER {
                let job_id = (i * JOBS_PER_PRODUCER + j) as u64;
                queue_clone.push(Job::new(job_id));
            }
        });
        handles.push(producer_handle);
    }

    // --- 3. Запуск Потребителей (Consumers) ---
    for _ in 0..NUM_CONSUMERS {
        let queue_clone = Arc::clone(&queue);
        let processed_clone = Arc::clone(&jobs_processed);
        
        let consumer_handle = thread::spawn(move || {
            loop {
                match queue_clone.pop() {
                    Some(_job) => {
                        // Инкрементируем счетчик, когда задача успешно извлечена
                        processed_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    },
                    None => {
                        // Очередь пуста. Проверяем, достигнут ли общий лимит.
                        if processed_clone.load(std::sync::atomic::Ordering::SeqCst) >= TOTAL_JOBS {
                            break; 
                        }
                        // Mutex более эффективно использует блокировку ядра, поэтому yield_now() 
                        // здесь может быть менее критичен, но оставим для справедливости сравнения.
                        thread::yield_now(); 
                    }
                }
            }
        });
        handles.push(consumer_handle);
    }

    // --- 4. Ожидание завершения работы ---
    
    // 4.1 Ждем завершения ВСЕХ производителей.
    println!("Ждем завершения производителей...");
    for _ in 0..NUM_PRODUCERS {
        handles.remove(0).join().unwrap();
    }
    
    // 4.2 Ждем завершения ВСЕХ потребителей.
    println!("Ждем завершения потребителей...");
    for _ in 0..NUM_CONSUMERS {
         handles.remove(0).join().unwrap();
    }

    // 5. Проверка результата
    let final_processed = jobs_processed.load(std::sync::atomic::Ordering::SeqCst);
    println!("Финальное количество обработанных задач: {}", final_processed);
    println!("Ожидаемое количество задач: {}", TOTAL_JOBS);
    assert_eq!(final_processed, TOTAL_JOBS, "Не все задачи были обработаны.");
}
