# 📊 SpinLock/Mutex Performance Metrics Subsystem

This repository contains the Rust implementation for a robust subsystem designed to collect and analyze **performance metrics** for custom synchronization primitives, specifically our `SpinLock`, compared against the standard library's `std::sync::Mutex`.

The project demonstrates how integrated metrics reveal the true cost of synchronization and how effective optimization techniques (like thread yielding) are in a highly concurrent environment.

## 🛠️ Implementation and Architecture

The core of the project involves two custom components:

1.  **`SpinLock<T>` (Custom):** A high-performance spinlock using `AtomicBool` and memory ordering primitives.
2.  **`LockMetrics` Subsystem:** A centralized structure to accumulate detailed statistics during the lock/unlock cycle.
    
### Key Metrics Collected

| Metric | Description |
| :--- | :--- |
| **Lock Count** | Total number of successful lock acquisitions. |
| **Spin Count** | Total number of CPU cycles spent actively waiting (spinning) for the lock to be free. |
| **Wait NS** | Total time (in nanoseconds) threads spent waiting to acquire the lock. |
| **Contentions** | The number of times a thread attempted to acquire a lock that was already held. |
| **Average Spins/Contention** | Calculated ratio (`Spin Count / Contentions`) showing the efficiency of the spinning strategy. |
| **Critical Section Time (Hold Time)** | **[FUTURE WORK]** The time the lock is actively held, planned for implementation in the next step. |

## 🚀 Performance Analysis and Optimization

The following data was collected from the test `spinlock_test.rs` which runs **4 threads** performing **10,000 increments** each on a shared counter, simulating high contention.

### 1. Initial SpinLock (Active Spinning)

This baseline implementation used simple active spinning (`spin_loop()`) without yielding control to the OS scheduler.

| Metric | Initial SpinLock Value | Interpretation |
| :--- | :--- | :--- |
| **Total Spin Count** | **363,589** | Extremely high resource waste, as threads consume CPU cycles while waiting. |
| **Total Contentions** | 33,501 | High competition for the lock. |
| **Average Spins/Contention** | 10.85 | Every time the lock was busy, the thread spent ~11 cycles spinning before gaining access. |
| **Test Duration (`spinlock_test.rs`)** | **0.02s** | Moderate performance due to excessive CPU load from spinning. |

***

### 2. Optimized SpinLock (With Thread Yielding)

The `SpinLock` was optimized by introducing **`thread::yield_now()`** after 10 failed spin attempts (`SPIN_LIMIT_BEFORE_YIELD = 10`). This optimization instructs the OS scheduler to run other threads, drastically reducing CPU waste.

| Metric | Optimized SpinLock Value | Improvement |
| :--- | :--- | :--- |
| **Total Spin Count** | **76,906** | **↓ 78.8\% decrease** |
| **Total Contentions** | **13,034** | **↓ 61.1\% decrease** (The lock is released faster) |
| **Average Spins/Contention** | **5.90** | **↓ 45.6\% decrease** (More efficient use of spinning) |
| **Test Duration (`spinlock_test.rs`)** | **0.01s** | **2x faster** |

**Conclusion:** The metrics clearly show that while SpinLocks are **Wait-Free** in theory, they must yield to the scheduler under high contention to avoid massive CPU overhead. The introduction of `yield_now()` improved performance and reduced wasted CPU cycles dramatically.

***

### 3. Mutex vs. SpinLock Comparison (Producer/Consumer Test)

The test `queue_test.rs` and `mutex_queue_test.rs` used a Producer/Consumer pattern (4 Prods / 4 Cons / 40,000 Jobs) to compare the custom `SpinLockJobQueue` against the `MutexJobQueue` (using `std::sync::Mutex`).

| Synchronization Primitive | Test Duration | Best Use Case |
| :--- | :--- | :--- |
| **`std::sync::Mutex`** | **0.01s** | High Contention, I/O-heavy, or long critical sections. (It utilizes the OS scheduler efficiently). |
| **Optimized `SpinLock`** | **0.08s** | Low Contention, extremely short critical sections where the overhead of OS context switching must be avoided. |

## 🧹 Repository State and Future Work

This repository was recently initialized via a **force push (`git push -f`)** to ensure a clean history focused purely on the development of the metrics system, removing all extraneous initial files and commits.

### Next Step: Completing the Metrics Subsystem

The immediate next step is to implement the measurement of **Critical Section Time (Hold Time)** within the `SpinGuard`'s `Drop` implementation to fully capture the performance profile of the locking primitive.
