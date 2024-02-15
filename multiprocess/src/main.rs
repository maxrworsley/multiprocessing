use std::{thread, time::{Duration, Instant}};

const ITERATIONS_OF_WORK: i32 = 8;

fn main() {
    testing_wrapper(test_single, "Single".to_string())
}

fn testing_wrapper(f: fn() -> (), name: String) {
    let start = Instant::now();
    f();
    let duration_seconds = start.elapsed().as_secs();
    let duration_milli = start.elapsed().as_nanos();

    println!("{}: {:?}.{:?}", name, duration_seconds, duration_milli);
}

fn test_single() {
    for i in 0..ITERATIONS_OF_WORK {
        busy_work();
    }
}

fn test_multi() {
    // let mut thread_pool = Vec::new();
    let max_thread_count = 8;

    let mut work_left = ITERATIONS_OF_WORK;
    let current_thread_count = 0;
    // Use mpsc channels to hand work to threads
    // Will need channels both ways - one for work and one for ready signal
    
    while work_left > 0 {
        let thread_deficit = max_thread_count - current_thread_count;
        for _ in 0..thread_deficit {
            // Push work to channels
        }        
    }
}

fn busy_work() -> i64 {
    let mut x: i64 = 0;
    for i in 0..10_000_000 {
        x += i;
    }
    return x
}
