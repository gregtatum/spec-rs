extern crate rand;

use std::time::Duration;
use std::thread;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

fn the_work() -> Vec<u8> {
    Vec::with_capacity(10240000)
}

fn main() {
    let mut vector = the_work();
    let mut rng = rand::thread_rng();
    for i in 0..10240000 {
        vector[i] = rng.gen();
    }
    loop {
        let between = Range::new(0, 1024000);

        thread::sleep(Duration::from_millis(100));
        println!("Hello, world! {:?}", vector[between.ind_sample(&mut rng)]);
    }
}
