# Result

```bash
$ cargo nextest run
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
    Starting 6 tests across 2 binaries
        PASS [   2.020s] singly_linked02 tests::it_works
        PASS [   1.660s] singly_linked02 tests::list_peak
        PASS [   1.299s] singly_linked02 tests::list_pop
        PASS [   0.938s] singly_linked02 tests::list_push
        PASS [   0.622s] singly_linked02 tests::peak_test
        PASS [   0.330s] singly_linked02 tests::push_test
------------
     Summary [   2.041s] 6 tests run: 6 passed, 0 skipped
```


- result 2

```bash
[src/main.rs:7] list = LinkedList {
    head: Some(
        Node {
            element: 1,
            next: Some(
                Node {
                    element: 1024,
                    next: None,
                },
            ),
        },
    ),
}
[src/main.rs:12] list02.peak() = Some(
    256,
)
[src/main.rs:13] list02 = LinkedList {
    head: Some(
        Node {
            element: 256,
            next: Some(
                Node {
                    element: 2048,
                    next: None,
                },
            ),
        },
    ),
}
```
