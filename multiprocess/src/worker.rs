use std::sync::mpsc::{Receiver, Sender};

pub struct ReadyForWork;

pub struct Worker {
    work_queue: Receiver<fn()->i64>,
    ready_for_work: Sender<ReadyForWork>
}

impl Worker {
    pub fn new(work_queue: Receiver<fn()->i64>, ready_for_work: Sender<ReadyForWork>) -> Worker {
        Worker { work_queue, ready_for_work }
    }

    pub fn run(&mut self) {
        // As part of init we need to say we are ready to work
        if let Err(_) = self.ready_for_work.send(ReadyForWork{}) {
            return
        }

        loop {
            if let Ok(work) = self.work_queue.recv() {
                work();
                if let Ok(_) = self.ready_for_work.send(ReadyForWork{}) {
                    continue;
                }
            }
            return;
        }
    }
}