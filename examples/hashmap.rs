#![allow(unused_variables)]

trait Hashable {
    fn hash(&self, key_size: usize) -> usize;
}

impl Hashable for i32 {
    fn hash(&self, key_size: usize) -> usize {
        // So silly, but it's simple and works.
        (self.abs() % (key_size as i32)) as usize
    }
}

#[derive(Debug)]
struct Collision<K, V> {
    pub key: K,
    pub value: V,
    pub next: Option<Box<Collision<K, V>>>
}

impl<K: Hashable + Eq, V> Collision<K, V> {
    fn new(key: K, value: V) -> Collision<K, V> {
        Collision {
            key: key,
            value: value,
            next: None
        }
    }

    fn add(&mut self, key: K, value: V) -> bool {
        if self.key != key {
            match self.next {
                Some(ref mut collision) => {
                    return collision.add(key, value);
                },
                None => {
                    self.next = Some(Box::new(Collision::new(key, value)));
                    return true;
                }
            }
        }
        false
    }

    fn has(&self, key: K) -> bool {
        if self.key == key {
            return true;
        }
        match self.next {
            Some(ref collision) => collision.has(key),
            None => false
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        if self.key == key {
            return Some(&self.value);
        }
        match self.next {
            Some(ref collision) => collision.get(key),
            None => None
        }
    }

}

#[derive(Debug)]
struct HashMap<K, V> {
    keys: Vec<Option<Collision<K, V>>>,
    len: usize,
}

impl<K: Hashable + Eq, V> HashMap<K, V> {
    fn new(key_size: usize) -> HashMap<K, V> {
        let mut keys = Vec::with_capacity(key_size);
        for i in 0..key_size {
            keys.push(None);
        }
        HashMap {
            len: 0,
            keys: keys,
        }
    }

    fn add(&mut self, key: K, value: V) {
        let hash = key.hash(self.keys.len());

        match self.keys.get_mut(hash).unwrap() {
            &mut Some(ref mut collision) => {
                if collision.add(key, value) {
                    self.len += 1;
                }
                return;
            },
            _ => {},
        }
        self.keys[hash] = Some(Collision::new(key, value));
        self.len += 1;
    }

    fn has(&self, key: K) -> bool {
        let hash = key.hash(self.keys.capacity());
        match self.keys.get(hash).unwrap() {
            &Some(ref collision) => collision.has(key),
            &None => false,
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        let hash = key.hash(self.keys.capacity());
        match self.keys.get(hash).unwrap() {
            &Some(ref collision) => collision.get(key),
            &None => None,
        }
    }

}

fn main() {
    test_simple_mode_hashing();
    test_hashmap_length();
    test_hashmap_has();
    test_hashmap_get();
    println!("Done running hashmap code.");
}

fn test_simple_mode_hashing() {
    let key_size = 12;
    // Run some simple tests on the code.
    assert_eq!(simple_mod_hash(100, key_size), 4);
    assert_eq!(simple_mod_hash(-100, key_size), 4);
    assert_eq!(simple_mod_hash(11, key_size), 11);
    assert_eq!(simple_mod_hash(12, key_size), 0);
}

fn test_hashmap_length() {
    let mut set = HashMap::new(200);
    assert_eq!(set.len, 0);

    set.add(50, 0);
    assert_eq!(set.len, 1);

    set.add(51, 1);
    assert_eq!(set.len, 2);

    // With simple modulo hashing, this will be a collision with 50.
    set.add(250, 2);
    assert_eq!(set.len, 3);
}

fn test_hashmap_has() {
    let set_key_size = 200;
    assert_eq!(
        simple_mod_hash(50, set_key_size),
        simple_mod_hash(250, set_key_size),
        "This test assumes that 50 and 250 are hash collisions"
    );

    let mut set = HashMap::new(set_key_size);
    assert_eq!(set.has(50), false, "Initially the set doesn't have these values.");
    assert_eq!(set.has(51), false, "Initially the set doesn't have these values.");
    assert_eq!(set.has(250), false, "Initially the set doesn't have these values.");

    set.add(50, 0);
    set.add(51, 1);
    set.add(250, 2);

    assert_eq!(set.has(50), true, "Checking for added numbers works.");
    assert_eq!(set.has(51), true, "Checking for added numbers works.");
    assert_eq!(set.has(250), true, "Checking for numbers with collisions work..");

    // This is a collision with both 50 and 250.
    assert_eq!(set.has(450), false);
}

fn test_hashmap_get() {
    let mut set = HashMap::new(200);

    set.add(50, 0);
    set.add(51, 1);
    set.add(250, 2);

    assert_eq!(*set.get(50).unwrap(), 0, "Got a value out");
    assert_eq!(*set.get(51).unwrap(), 1, "Got a value out");
    assert_eq!(*set.get(250).unwrap(), 2, "Got a value out");
}
