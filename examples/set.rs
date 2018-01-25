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

    fn has(&self, value: i32) -> bool {
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
struct Set {
    keys: Vec<Option<Collision>>,
    len: usize,
    // iterator_key_index: usize,
    // iterator_collision_depth: usize,
}

impl Set {
    fn new(key_size: usize) -> Set {
        let mut keys = Vec::with_capacity(key_size);
        for i in 0..key_size {
            keys.push(None);
        }
        Set {
            len: 0,
            keys: keys,
            // iterator_key_index: 0,
            // iterator_collision_depth: 0,
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

    fn has(&self, number: i32) -> bool {
        let hash = simple_mod_hash(number, self.keys.capacity()) as usize;
        match self.keys.get(hash).unwrap() {
            &Some(ref collision) => collision.has(number),
            &None => false,
        }
    }
}

// impl Iterator for Set {
//     type Item = Box<i32>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // Continue searching through the keys
//         for key in (self.iterator_position)..(self.keys.len()) {
//             match key {
//                 Some(ref collision) => {
//                     for iterator_depth in  {
//                         iterator_collision_depth
//                     }
//                 },
//                 None => {}
//             }
//             // Reset the collision depth to start back at the top.
//             self.iterator_collision_depth = 0;
//             self.iterator_position += 1;
//         }
//
//         if self.iterator_position < 6 {
//             Some(self.iterator_position)
//         } else {
//             None
//         }
//     }
// }

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
