use std::iter;

use crossbeam_deque::{Injector, Stealer, Worker};

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

#[cfg(test)]

mod tests {
    use super::*;
    use crossbeam_deque::Steal;

    #[test]
    fn it_works() {
        let q = Injector::new();
        q.push(1);
        q.push(2);
        dbg!(&q);
        assert_eq!(q.steal(), Steal::Success(1));
        assert_eq!(q.steal(), Steal::Success(2));
        assert_eq!(q.steal(), Steal::Empty);
        dbg!(q);

        let w = Worker::new_lifo();
        let collect = w.stealer();

        w.push(1);
        w.push(2);
        w.push(3);
        dbg!(&w);

        assert_eq!(collect.steal(), Steal::Success(1));
        assert_eq!(w.pop(), Some(3));
        assert_eq!(w.pop(), Some(2));
        let q = Injector::new();
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);
        q.push(6);

        let w = Worker::new_fifo();
        let _ = q.steal_batch_with_limit(&w, 2);
        assert_eq!(w.pop(), Some(1));
        assert_eq!(w.pop(), Some(2));
        assert_eq!(w.pop(), None);

        q.push(7);
        q.push(8);
        dbg!(&q);
        // Setting a large limit does not guarantee that all elements will be popped. In this case,
        // half of the elements are currently popped, but the number of popped elements is considered
        // an implementation detail that may be changed in the future.
        let _ = q.steal_batch_with_limit(&w, std::usize::MAX);
        assert_eq!(w.len(), 3);
        //
        //
    }
    #[test]
    fn it_works_find_task() {
        let local_worker: Worker<i32> = Worker::new_lifo();
        let global_injector = Injector::new();
        let mut stealers: Vec<Stealer<_>> = vec![];
        local_worker.push(1);
        local_worker.push(2);
        local_worker.push(3);
        local_worker.push(9);
        global_injector.push(1);
        global_injector.push(2);
        global_injector.push(9);
        global_injector.push(3);
        stealers.pop();
        stealers.pop();

        let result = find_task(&local_worker, &global_injector, &stealers);
        dbg!(result);
    }
    // Using find_task function
}
