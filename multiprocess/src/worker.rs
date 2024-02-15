use std::sync::mpsc::{Receiver, Sender};

pub struct ReadyForWork;

pub struct Worker {
    job_queue: Receiver<fn()->i64>,
    ready_for_job: Sender<ReadyForWork>
}

impl Worker {
    pub fn new(job_queue: Receiver<fn()->i64>, ready_for_job: Sender<ReadyForWork>) -> Worker {
        Worker { job_queue, ready_for_job }
    }

    pub fn run(&mut self) {
        // As part of init we need to say we are ready to work
        if let Err(_) = self.ready_for_job.send(ReadyForWork{}) {
            return
        }

        loop {
            if let Ok(job) = self.job_queue.recv() {
                job();
                if let Ok(_) = self.ready_for_job.send(ReadyForWork{}) {
                    continue;
                }
            }
            return;
        }
    }
}