use crossbeam_deque::{Injector, Stealer, Worker};

struct ThreadData<Task> {
    injector: Injector<Task>,     // common global queue
    task_q: Worker<Task>,         // local queue
    stealers: Vec<Stealer<Task>>, // stealers for other threads local queue
}

impl<Task> ThreadData<Task> {
    fn spawn(mut self) -> Option<std::thread::JoinHandle<()>> {
        let thread = std::thread::spawn(move || {
            find_task(&self.task_q, &self.injector, &self.stealers);
        });
        Some(thread)
    }
}

struct Worker<Task> {
    stealer: Stealer<Task>, // to be shared with other threads
    thread: Option<std::thread::JoinHandle<()>>,
}

struct Factory<Task> {
    injector: Injector<Task>, // owner of the global queue
    workers: Vec<Worker<Task>>,
}

impl<Task> Factory<Task> {
    fn new() -> Self {
        Self {
            injector: Injector::<Task>::new(),
            workers: Vec::new(),
        }
    }

    fn build_threadpool(&mut self) {
        let mut t1 = ThreadData {
            injector: self.injector.clone(),
            task_q: Worker::<Task>::new_fifo(),
            stealers: Vec::new(),
        };
        let w1 = Worker {
            stealer: t1.task_q.stealer(),
            thread: None,
        };

        let mut t2 = ThreadData {
            injector: self.injector.clone(),
            task_q: Worker::<Task>::new_fifo(),
            stealers: Vec::new(),
        };
        let w2 = Worker {
            stealer: t2.task_q.stealer(),
            thread: None,
        };

        let mut t3 = ThreadData {
            injector: self.injector.clone(),
            task_q: Worker::<Task>::new_fifo(),
            stealers: Vec::new(),
        };
        let w3 = Worker {
            stealer: t3.task_q.stealer(),
            thread: None,
        };

        t1.stealers.push(w2.stealer.clone());
        t1.stealers.push(w3.stealer.clone());

        t2.stealers.push(w1.stealer.clone());
        t2.stealers.push(w3.stealer.clone());

        t3.stealers.push(w1.stealer.clone());
        t3.stealers.push(w2.stealer.clone());

        // launch threads and save workers
        w1.thread = t1.spawn();
        w2.thread = t2.spawn();
        w3.thread = t3.spawn();

        self.workers.push(w1);
        self.workers.push(w2);
        self.workers.push(w3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut factory = Factory::<i32>::new();
        factory.build_threadpool();
        // Add assertions or other test logic as needed
    }
}
