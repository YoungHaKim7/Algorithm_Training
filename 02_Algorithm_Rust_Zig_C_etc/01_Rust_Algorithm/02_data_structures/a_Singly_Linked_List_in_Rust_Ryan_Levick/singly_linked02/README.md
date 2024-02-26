# Result

```
$ cargo nextest run        
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
    Starting 5 tests across 2 binaries
        PASS [   1.620s] singly_linked02 tests::it_works
        PASS [   1.309s] singly_linked02 tests::list_peak
        PASS [   0.997s] singly_linked02 tests::list_pop
        PASS [   0.683s] singly_linked02 tests::list_push
        PASS [   0.387s] singly_linked02 tests::push_test
------------
     Summary [   1.639s] 5 tests run: 5 passed, 0 skipped
```


- result 2

```
$ cargo r          
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/singly_linked02`

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
```
