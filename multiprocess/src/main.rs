use std::{
    sync::mpsc, thread, time::{
        Duration, 
        Instant}};

use worker::Worker;
mod worker;

const ITERATIONS_OF_WORK: i32 = 64;

fn main() {
    testing_wrapper(test_single, "Single".to_string());
    testing_wrapper(test_multi, "Multi".to_string());
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
    let max_thread_count = 8;
    let mut worker_threads = Vec::new();
    let mut work_sending_channels = Vec::new();
    let mut ready_signals = Vec::new();

    for _ in 0..max_thread_count {
        let (work_sender, work_receiver) = mpsc::channel();
        let (ready_sender, ready_receiver) = mpsc::channel();
        work_sending_channels.push(work_sender);
        ready_signals.push(ready_receiver);
        let mut new_worker = Worker::new(work_receiver, ready_sender);
        worker_threads.push(thread::spawn(move || new_worker.run()));
    }

    let mut work_left = ITERATIONS_OF_WORK;
    
    while work_left > 0 {
        for i in 0..max_thread_count {
            if let Ok(_) = ready_signals[i].try_recv() {
                match work_sending_channels[i].send(busy_work) {
                    Ok(_) => work_left -= 1,
                    Err(_) => break,
                }
            }
        }
    }

    drop(work_sending_channels);
    for handle in worker_threads {
        handle.join().unwrap();
    }
}

fn busy_work() -> i64 {
    let mut x: i64 = 0;
    for i in 0..10_000_000 {
        x += i;
    }
    return x
}
