# Result

- `cargo t -- --nocapture`

```

$ cargo t -- --nocapture

   Compiling a01_deque_crossbeam v0.1.0 (/Users/gy-gyoung/my_project/Rust_Lang/Algorithm_Training/02_Algorithm_Rust_Zig_C_etc/01_Rust_Algorithm/02_data_structures/Deque_crossbeam/a01_deque_crossbeam)
warning: function `find_task` is never used
 --> src/lib.rs:5:4
  |
5 | fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `a01_deque_crossbeam` (lib test) generated 1 warning
warning: `a01_deque_crossbeam` (lib) generated 1 warning (1 duplicate)
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/a01_deque_crossbeam-2e262533161c7a39)

running 1 test
[src/lib.rs:28:9] &q = Worker { .. }
[src/lib.rs:32:9] q = Worker { .. }
[src/lib.rs:40:9] &w = Worker { .. }
[src/lib.rs:61:9] &q = Worker { .. }
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests a01_deque_crossbeam

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
