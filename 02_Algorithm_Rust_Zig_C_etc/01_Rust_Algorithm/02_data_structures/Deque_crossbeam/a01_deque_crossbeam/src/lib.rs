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

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let q = Injector::new();
        q.push(1);
        q.push(2);
        //
        assert_eq!(q.steal(), Steal::Success(1));
        assert_eq!(q.steal(), Steal::Success(2));
        assert_eq!(q.steal(), Steal::Empty);
    }
}
