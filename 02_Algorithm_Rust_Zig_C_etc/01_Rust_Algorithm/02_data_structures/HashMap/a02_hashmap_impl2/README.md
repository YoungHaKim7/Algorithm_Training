# Result

```bash
Hello, Hashmap new push etc...!
Hello, Hashmap2 Test new push etc...!
cache check : LRUCache {
    capacity: 128,
    head: Some(
        5,
    ),
    tail: Some(
        0,
    ),
    map: {
        "foo": 0,
        "bar": 1,
        "corge": 5,
        "qux": 3,
        "quux": 4,
        "baz": 2,
    },
    entries: [
        Entry {
            key: "foo",
            val: Some(
                1,
            ),
            next: Some(
                1,
            ),
            prev: None,
        },
        Entry {
            key: "bar",
            val: Some(
                2,
            ),
            next: Some(
                2,
            ),
            prev: None,
        },
        Entry {
            key: "baz",
            val: Some(
                3,
            ),
            next: Some(
                3,
            ),
            prev: None,
        },
        Entry {
            key: "qux",
            val: Some(
                4,
            ),
            next: Some(
                4,
            ),
            prev: None,
        },
        Entry {
            key: "quux",
            val: Some(
                5,
            ),
            next: Some(
                5,
            ),
            prev: None,
        },
        Entry {
            key: "corge",
            val: Some(
                6,
            ),
            next: Some(
                4,
            ),
            prev: None,
        },
    ],
}
empty_check: false
is full check: false
cache check : LRUCache {
    capacity: 128,
    head: Some(
        2,
    ),
    tail: Some(
        0,
    ),
    map: {
        "foo": 0,
        "bar": 1,
        "corge": 5,
        "qux": 3,
        "quux": 4,
        "baz": 2,
    },
    entries: [
        Entry {
            key: "foo",
            val: Some(
                1,
            ),
            next: None,
            prev: None,
        },
        Entry {
            key: "bar",
            val: Some(
                2,
            ),
            next: Some(
                2,
            ),
            prev: None,
        },
        Entry {
            key: "baz",
            val: Some(
                3,
            ),
            next: Some(
                3,
            ),
            prev: None,
        },
        Entry {
            key: "qux",
            val: Some(
                4,
            ),
            next: Some(
                4,
            ),
            prev: None,
        },
        Entry {
            key: "quux",
            val: Some(
                5,
            ),
            next: Some(
                5,
            ),
            prev: None,
        },
        Entry {
            key: "corge",
            val: Some(
                6,
            ),
            next: Some(
                4,
            ),
            prev: None,
        },
    ],
}
quux: Some(
    5,
)
```

