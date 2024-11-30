# Result

```bash
Hello, HashMap new push etc..
Hello, HashMap new push etc 2..
LRUCache {
    cap: 100,
    head: Some(
        5,
    ),
    tail: Some(
        0,
    ),
    map: {
        "baz": 2,
        "foo": 0,
        "bar": 1,
        "corge": 5,
        "qux": 3,
        "quux": 4,
    },
    entries: [
        Entry {
            key: "foo",
            val: Some(
                1,
            ),
            next: None,
            prev: Some(
                1,
            ),
        },
        Entry {
            key: "bar",
            val: Some(
                2,
            ),
            next: Some(
                0,
            ),
            prev: Some(
                2,
            ),
        },
        Entry {
            key: "baz",
            val: Some(
                3,
            ),
            next: Some(
                1,
            ),
            prev: Some(
                3,
            ),
        },
        Entry {
            key: "qux",
            val: Some(
                4,
            ),
            next: Some(
                2,
            ),
            prev: Some(
                4,
            ),
        },
        Entry {
            key: "quux",
            val: Some(
                5,
            ),
            next: Some(
                3,
            ),
            prev: Some(
                5,
            ),
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
~~~empty check~~
false
~~is _ full check~~~
false


```

