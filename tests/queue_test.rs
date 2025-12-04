// tests/queue_test.rs

use spinmetrics::JobQueue;
use spinmetrics::Job;
use std::sync::Arc;
use std::thread;

/// Количество потоков-производителей (Producer Threads)
const NUM_PRODUCERS: usize = 4;
/// Количество потоков-потребителей (Consumer Threads)
const NUM_CONSUMERS: usize = 4;
/// Количество задач, которые генерирует каждый производитель
const JOBS_PER_PRODUCER: u64 = 10_000;
/// Общее количество задач
const TOTAL_JOBS: u64 = NUM_PRODUCERS as u64 * JOBS_PER_PRODUCER;

#[test]
fn producer_consumer_integration_test() {
    println!("Запуск симуляции Producer/Consumer:");
    println!("Производителей: {}", NUM_PRODUCERS);
    println!("Потребителей: {}", NUM_CONSUMERS);
    println!("Всего задач: {}", TOTAL_JOBS);

    // 1. Создаем общую очередь, защищенную Arc для доступа из разных потоков.
    let queue = Arc::new(JobQueue::new());

    let mut handles = vec![];
    let jobs_processed = Arc::new(std::sync::atomic::AtomicU64::new(0));

    // --- 2. Запуск Производителей (Producers) ---
    for i in 0..NUM_PRODUCERS {
        let queue_clone = Arc::clone(&queue);
        let producer_handle = thread::spawn(move || {
            for j in 0..JOBS_PER_PRODUCER {
                // Генерируем уникальный ID для каждой задачи
                let job_id = (i as u64 * JOBS_PER_PRODUCER) + j;
                let job = Job::new(job_id);
                queue_clone.push(job);
            }
        });
        handles.push(producer_handle);
    }
// ...
    // --- 3. Запуск Потребителей (Consumers) ---
    // Потребители будут крутиться до тех пор, пока не обработают все 40k задач.
    for _ in 0..NUM_CONSUMERS {
        let queue_clone = Arc::clone(&queue);
        let processed_clone = Arc::clone(&jobs_processed);
        
        let consumer_handle = thread::spawn(move || {
            loop {
                match queue_clone.pop() {
                    Some(_job) => {
                        // Только инкрементируем счетчик, когда задача успешно извлечена
                        processed_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    },
                    // Очередь пуста.
                    None => {
                        // Проверяем, достигнут ли общий лимит. 
                        // Это условие становится истинным только после того, как ВСЕ 
                        // потоки-производители закончили и все остальные потоки-потребители
                        // уже успели инкрементировать счетчик до TOTAL_JOBS.
                        if processed_clone.load(std::sync::atomic::Ordering::SeqCst) >= TOTAL_JOBS {
                            break; 
                        }
                        // Даем процессору "отдохнуть"
                        thread::yield_now(); 
                    }
                }
            }
        });
        handles.push(consumer_handle);
    }
    
    // --- 4. Ожидание завершения работы ---
    
    // 4.1 Ждем завершения ВСЕХ производителей.
    // Это гарантирует, что 40000 задач УЖЕ находятся в очереди.
    println!("Ждем завершения производителей...");
    for i in 0..NUM_PRODUCERS {
        // Мы используем _i чтобы избежать warning: unused variable
        let _ = i; 
        handles.remove(0).join().unwrap();
    }
    
    // 4.2 Ждем завершения ВСЕХ потребителей.
    // Теперь потребители могут быть уверены, что все задачи поступили,
    // и они должны работать до тех пор, пока счетчик не достигнет TOTAL_JOBS.
    println!("Ждем завершения потребителей...");
    for _ in 0..NUM_CONSUMERS {
         handles.remove(0).join().unwrap();
    }
    // ...
    // --- 5. Проверка результатов ---
    let final_processed_count = jobs_processed.load(std::sync::atomic::Ordering::SeqCst);

    println!("Финальное количество обработанных задач: {}", final_processed_count);
    println!("Ожидаемое количество задач: {}", TOTAL_JOBS);

    assert_eq!(final_processed_count, TOTAL_JOBS, 
               "Обнаружена потеря задач! Ожидалось {}, обработано {}", 
               TOTAL_JOBS, final_processed_count);
}
