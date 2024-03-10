// details on injector, worker and stealer can be found here:
// https://docs.rs/crossbeam/0.7.3/crossbeam/deque/index.html

use std::iter;

use crossbeam_deque::{Injector, Steal, Stealer, Worker};

fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
    local.pop().or_else(|| {
        iter::repeat_with(|| {
            global
                .steal_batch_and_pop(local)
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        .find(|s| s.is_retry())
        .and_then(|s| s.success())
    })
}

struct ThreadData<'a, Task> {
    injector: &'a Injector<Task>, // common global queue
    task_q: Worker<Task>,         // local queue
    stealers: [Stealer<Task>],    // stealers for other threads local queue
}
impl<Task: std::marker::Send> ThreadData<'_, Task> {
    fn spawn(&self) -> Option<std::thread::JoinHandle<()>> {
        let thread = std::thread::spawn(move || {
            find_task(&self.task_q, &self.injector, &self.stealers);
        });
        Some(thread)
    }
}
// struct Worker<Task> {
//     stealer: deque::Stealer<Task>, // to be shared with other threads
//     thread: Option<std::thread::JoinHandle<()>>,
// }

struct Factory<Task> {
    injector: deque::Injector<Task>, // owner of the global queue
    workers: Vec<Worker<Task>>,
}
impl<Task> Factory<Task> {
    fn new() -> Self {
        Self {
            injector: Injector::<Task>::new(),
            workers: Vec::new(),
        }
    }
    fn build_threadpool(mut self) {
        let mut t1 = ThreadData {
            injector: &self.injector,
            task_q: Worker::<Task>::new_fifo(),
            stealers: Vec::new(),
        };
        let w1 = Worker {
            stealer: t1.task_q.stealer(),
            thread: None,
        };

        let t2 = ThreadData {
            injector: &self.injector,
            task_q: Worker::<Task>::new_fifo(),
            stealers: Vec::new(),
        };
        let w2 = Worker {
            stealer: t2.task_q.stealer(),
            thread: None,
        };

        let t3 = ThreadData {
            injector: &self.injector,
            task_q: Worker::<Task>::new_fifo(),
            stealers: Vec::new(),
        };
        let w3 = Worker {
            stealer: t3.task_q.stealer(),
            thread: None,
        };

        t1.stealers.push(&w2.stealer);
        t1.stealers.push(&w3.stealer);

        t2.stealers.push(&w1.stealer);
        t2.stealers.push(&w3.stealer);

        t3.stealers.push(&w1.stealer);
        t3.stealers.push(&w2.stealer);

        // launch threads and save workers
        w1.thread = t1.spawn();
        w2.thread = t2.spawn();
        w3.thread = t3.spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
