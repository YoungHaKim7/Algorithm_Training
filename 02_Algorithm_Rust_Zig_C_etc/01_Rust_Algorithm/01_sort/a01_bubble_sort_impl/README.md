# Result

```bash
$ cargo nextest run

warning: function `sort` is never used
 --> src/lib.rs:7:4
  |
7 | fn sort<T, S>(slice: &mut [T])
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `a01_bubble_sort_impl` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 2 tests across 1 binary
        PASS [   0.004s] a01_bubble_sort_impl tests::std_works
        PASS [   0.004s] a01_bubble_sort_impl bubblesort::it_works
------------
     Summary [   0.004s] 2 tests run: 2 passed, 0 skipped

```
