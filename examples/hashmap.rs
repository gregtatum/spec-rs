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

    // Returns the count of Collision structs added, either 0 or 1.
    fn set(&mut self, key: K, value: V) -> usize {
        if self.key == key {
            self.value = value;
            // No collisions added, this is a value update.
            0
        } else {
            match self.next {
                Some(ref mut collision) => {
                    return collision.set(key, value);
                },
                None => {
                    self.next = Some(Box::new(Collision::new(key, value)));
                    return 1;
                }
            }
        }
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
        for _ in 0..key_size {
            keys.push(None);
        }
        HashMap {
            len: 0,
            keys: keys,
        }
    }

    fn set(&mut self, key: K, value: V) {
        let hash = key.hash(self.keys.len());

        match self.keys.get_mut(hash).unwrap() {
            &mut Some(ref mut collision) => {
                self.len += collision.set(key, value);
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
    test_simple_modulo_hashing();
    test_hashmap_length();
    test_hashmap_has();
    test_hashmap_get();
    println!("Done running hashmap code.");
}

fn test_simple_modulo_hashing() {
    let key_size = 12;
    // Run some simple tests on the code.
    assert_eq!(100.hash(key_size), 4);
    assert_eq!((-100i32).hash(key_size), 4);
    assert_eq!(11.hash(key_size), 11);
    assert_eq!(12.hash(key_size), 0);
}

fn test_hashmap_length() {
    let mut hashmap = HashMap::new(200);
    assert_eq!(hashmap.len, 0);

    hashmap.set(50, 0);
    assert_eq!(hashmap.len, 1);

    hashmap.set(51, 1);
    assert_eq!(hashmap.len, 2);

    hashmap.set(250, 2);
    assert_eq!(hashmap.len, 3, "With simple modulo hashing, 250 will be a collision with 50.");

    hashmap.set(50, 3);
    assert_eq!(hashmap.len, 3, "Setting an existing value shouldn't increment the length.");
}

fn test_hashmap_has() {
    let hashmap_key_size = 200;
    assert_eq!(
        50.hash(hashmap_key_size),
        250.hash(hashmap_key_size),
        "This test assumes that 50 and 250 are hash collisions"
    );

    let mut hashmap = HashMap::new(hashmap_key_size);
    assert_eq!(hashmap.has(50), false, "Initially the hashmap doesn't have these values.");
    assert_eq!(hashmap.has(51), false, "Initially the hashmap doesn't have these values.");
    assert_eq!(hashmap.has(250), false, "Initially the hashmap doesn't have these values.");

    hashmap.set(50, 0);
    hashmap.set(51, 1);
    hashmap.set(250, 2);

    assert_eq!(hashmap.has(50), true, "Checking for added numbers works.");
    assert_eq!(hashmap.has(51), true, "Checking for added numbers works.");
    assert_eq!(hashmap.has(250), true, "Checking for numbers with collisions work..");

    // This is a collision with both 50 and 250.
    assert_eq!(hashmap.has(450), false);
}

fn test_hashmap_get() {
    let mut hashmap = HashMap::new(200);

    hashmap.set(50, 0);
    hashmap.set(51, 1);
    hashmap.set(250, 2);

    assert_eq!(*hashmap.get(50).unwrap(), 0, "Got a value out");
    assert_eq!(*hashmap.get(51).unwrap(), 1, "Got a value out");
    assert_eq!(*hashmap.get(250).unwrap(), 2, "Got a value out");

    hashmap.set(250, 4);
    assert_eq!(*hashmap.get(250).unwrap(), 4, "Values are updateable");
}
