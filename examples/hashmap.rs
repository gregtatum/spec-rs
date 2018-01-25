#![allow(unused_variables)]

// This is a super simple hashing function, but it does the trick.
fn simple_mod_hash(number_to_hash: i32, key_size: usize) -> i32 {
    number_to_hash.abs() % (key_size as i32)
}

#[derive(Debug)]
struct Collision {
    pub value: i32,
    pub next: Option<Box<Collision>>
}

impl Collision {
    fn new(value: i32) -> Collision {
        Collision {
            value: value,
            next: None
        }
    }

    fn add(&mut self, value: i32) -> bool {
        if self.value != value {
            match self.next {
                Some(ref mut collision) => {
                    return collision.add(value);
                },
                None => {
                    self.next = Some(Box::new(Collision::new(value)));
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct HashMap {
    len: usize,
    keys: Vec<Option<Collision>>
}

impl HashMap {
    fn new(key_size: usize) -> HashMap {
        let mut keys = Vec::with_capacity(key_size);
        for i in 0..key_size {
            keys.push(None);
        }
        HashMap {
            len: 0,
            keys: keys,
        }
    }

    fn add(&mut self, number: i32) {
        let hash = simple_mod_hash(number, self.keys.capacity()) as usize;

        let mut key_is_empty = false;
        match self.keys.get_mut(hash).unwrap() {
            &mut Some(ref mut collision) => {
                if collision.add(number) {
                    self.len += 1;
                }
            },
            &mut None => {
                // Defer updating this value to return the mutable borrow of `self.keys`.
                key_is_empty = true;
            },
        }
        if key_is_empty {
            self.keys[hash] = Some(Collision::new(number));
            self.len += 1;
        }
    }

    fn has(&mut self, number: i32) -> bool {
        true
    }
}

fn main() {
    test_simple_mode_hashing();
    test_create_hashmap();
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

fn test_create_hashmap() {
    let mut hash_map = HashMap::new(200);
    hash_map.add(50);
    hash_map.add(250);
    hash_map.has(50);
    println!("{:?}", hash_map);
}
