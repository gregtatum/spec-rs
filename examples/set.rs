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

// This is a super simple hashing function, but it does the trick.
fn simple_mod_hash(number_to_hash: i32, key_size: usize) -> i32 {
    number_to_hash.abs() % (key_size as i32)
}

#[derive(Debug)]
struct Collision<T> {
    pub value: T,
    pub next: Option<Box<Collision<T>>>
}

impl<T: Hashable + Eq> Collision<T> {
    fn new(value: T) -> Collision<T> {
        Collision {
            value: value,
            next: None
        }
    }

    fn add(&mut self, value: T) -> bool {
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

    fn has(&self, value: T) -> bool {
        if self.value == value {
            return true;
        }
        match self.next {
            Some(ref collision) => collision.has(value),
            None => false
        }
    }
}

#[derive(Debug)]
struct Set<T> {
    keys: Vec<Option<Collision<T>>>,
    len: usize,
}

impl<T: Hashable + Eq> Set<T> {
    fn new(key_size: usize) -> Set<T> {
        let mut keys = Vec::with_capacity(key_size);
        for i in 0..key_size {
            keys.push(None);
        }
        Set {
            len: 0,
            keys: keys,
        }
    }

    fn add(&mut self, number: T) {
        let hash = number.hash(self.keys.capacity());

        match self.keys.get_mut(hash).unwrap() {
            &mut Some(ref mut collision) => {
                if collision.add(number) {
                    self.len += 1;
                }
                return;
            },
            _ => {},
        }
        self.keys[hash] = Some(Collision::new(number));
        self.len += 1;
    }

    fn has(&self, number: T) -> bool {
        let hash = number.hash(self.keys.capacity());
        match self.keys.get(hash).unwrap() {
            &Some(ref collision) => collision.has(number),
            &None => false,
        }
    }
}

fn main() {
    test_simple_mode_hashing();
    test_set_length();
    test_set_has();
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

fn test_set_length() {
    let mut set = Set::new(200);
    assert_eq!(set.len, 0);

    set.add(50);
    assert_eq!(set.len, 1);

    set.add(51);
    assert_eq!(set.len, 2);

    // With simple modulo hashing, this will be a collision with 50.
    set.add(250);
    assert_eq!(set.len, 3);
}

fn test_set_has() {
    let set_key_size = 200;
    assert_eq!(
        simple_mod_hash(50, set_key_size),
        simple_mod_hash(250, set_key_size),
        "This test assumes that 50 and 250 are hash collisions"
    );

    let mut set = Set::new(set_key_size);
    assert_eq!(set.has(50), false, "Initially the set doesn't have these values.");
    assert_eq!(set.has(51), false, "Initially the set doesn't have these values.");
    assert_eq!(set.has(250), false, "Initially the set doesn't have these values.");

    set.add(50);
    set.add(51);
    set.add(250);

    assert_eq!(set.has(50), true, "Checking for added numbers works.");
    assert_eq!(set.has(51), true, "Checking for added numbers works.");
    assert_eq!(set.has(250), true, "Checking for numbers with collisions work..");

    // This is a collision with both 50 and 250.
    assert_eq!(set.has(450), false);
}
