use std::{collections::HashMap, hash::Hash};

const CHACHE_SIZE: usize = 100;

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    val: Option<V>,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug)]
struct LRUCache<K, V> {
    cap: usize,
    head: Option<usize>,
    tail: Option<usize>,
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Eq,
{
    fn new() -> Self {
        Self::with_capacity(CHACHE_SIZE)
    }

    fn with_capacity(cap: usize) -> Self {
        LRUCache {
            cap,
            head: None,
            tail: None,
            map: HashMap::with_capacity(cap),
            entries: Vec::with_capacity(cap),
        }
    }

    fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.map.contains_key(&key) {
            self.access(&key);
            let entry = &mut self.entries[self.head.unwrap()];
            let old_val = entry.val.take();
            entry.val = Some(val);
            old_val
        } else {
            self.ensure_room();

            let index = self.entries.len();
            if let Some(e) = self.head {
                self.entries[e].prev = Some(index);
            }
            //self.head.map(|e| {
            //    self.entries[e].prev = Some(index);
            //});

            self.entries.push(Entry {
                key: key.clone(),
                val: Some(val),
                prev: None,
                next: self.head,
            });
            self.head = Some(index);
            self.tail = self.tail.or(self.head);
            self.map.insert(key, index);

            None
        }
    }

    fn access(&mut self, key: &K) {
        let i = *self.map.get(key).unwrap();
        self.remove_from_list(i);
        self.head = Some(i)
    }

    fn remove_from_list(&mut self, i: usize) {
        let (prev, next) = {
            let entry = self.entries.get_mut(i).unwrap();
            (entry.prev, entry.next)
        };

        match (prev, next) {
            (Some(j), Some(k)) => {
                let head = &mut self.entries[j];
                head.next = next;
                let next = &mut self.entries[k];
                next.prev = prev;
            }

            (Some(j), None) => {
                let head = &mut self.entries[j];
                head.next = None;
                self.tail = prev;
            }

            _ => {
                if self.len() > 1 {
                    let head = &mut self.entries[0];
                    head.next = None;
                    let next = &mut self.entries[1];
                    next.prev = None;
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn ensure_room(&mut self) {
        if self.cap == self.len() {
            self.remove_tail();
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|index| {
            self.remove_from_list(index);
            self.entries[index].val.take().unwrap()
        })
    }

    fn remove_tail(&mut self) {
        if let Some(index) = self.tail {
            self.remove_from_list(index);
            let key = &self.entries[index].key;
            self.map.remove(key);
        }

        if self.tail.is_none() {
            self.head = None;
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &self.entries;
        self.map
            .get(key)
            .and_then(move |&i| entries[i].val.as_ref())
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &mut self.entries;
        self.map
            .get(key)
            .and_then(move |&i| entries[i].val.as_mut())
    }

    fn contains(&mut self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    fn empty(&mut self, _key: &K) -> bool {
        self.map.is_empty()
    }

    fn is_full(&self) -> bool {
        self.map.len() == self.cap
    }
}

fn main() {
    println!("Hello, HashMap new push etc..");
    let mut cache = LRUCache::with_capacity(2);
    cache.insert("foo", 1);
    cache.insert("bar", 2);

    cache.get(&"foo").unwrap();
    cache.insert("baz", 3);

    assert!(cache.contains(&"foo"));
    assert!(cache.contains(&"baz"));
    assert!(!cache.contains(&"bar"));

    cache.get_mut(&"foo").unwrap();
    cache.insert("qux", 4);

    assert!(cache.contains(&"foo"));
    assert!(!cache.contains(&"baz"));
    println!("Hello, HashMap new push etc 2..");
    let mut cache = LRUCache::with_capacity(2);
    cache.insert("foo", 1);

    assert!(!cache.contains(&"baz"));

    cache.remove(&"foo");
    let mut cache = LRUCache::new();
    cache.insert("foo", 1);
    cache.insert("bar", 2);
    cache.insert("baz", 3);
    cache.insert("qux", 4);
    cache.insert("quux", 5);
    cache.insert("corge", 6);
    println!("{:#?}", cache);
    let empty_check = cache.empty(&"foo");
    println!("~~~empty check~~");
    println!("{:#?}", empty_check);
    let is_fullval = cache.is_full();
    println!("~~is _ full check~~~");
    println!("{:?}", is_fullval);
}
