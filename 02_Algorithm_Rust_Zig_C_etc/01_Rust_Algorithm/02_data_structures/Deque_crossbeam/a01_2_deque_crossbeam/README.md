# Result


- `cargo nextest run --no-capture`

```bash

$ cargo nextest run --no-capture
warning: function `find_task` is never used
 --> src/lib.rs:5:4
  |
5 | fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `a01_2_deque_crossbeam` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 2 tests across 1 binary
       START             a01_2_deque_crossbeam tests::it_works

running 1 test
[src/lib.rs:28:9] &q = Worker { .. }
[src/lib.rs:32:9] q = Worker { .. }
[src/lib.rs:40:9] &w = Worker { .. }
[src/lib.rs:61:9] &q = Worker { .. }
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

        PASS [   0.004s] a01_2_deque_crossbeam tests::it_works
       START             a01_2_deque_crossbeam tests::it_works_find_task

running 1 test
[src/lib.rs:87:9] result = Some(
    9,
)
test tests::it_works_find_task ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

        PASS [   0.004s] a01_2_deque_crossbeam tests::it_works_find_task
------------
     Summary [   0.008s] 2 tests run: 2 passed, 0 skipped

```

- `cargo t -- --nocapture`

```bash

$ cargo t -- --nocapture
warning: function `find_task` is never used
 --> src/lib.rs:5:4
  |
5 | fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `a01_2_deque_crossbeam` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/a01_2_deque_crossbeam-e3dbcc07c1ca0420)

running 2 tests
[src/lib.rs:28:9] &q = Worker { .. }
[src/lib.rs:32:9] q = Worker { .. }
[src/lib.rs:87:9] result = Some(
    9,
)
[src/lib.rs:40:9] &w = Worker { .. }
[src/lib.rs:61:9] &q = Worker { .. }
test tests::it_works_find_task ... ok
test tests::it_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests a01_2_deque_crossbeam

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
