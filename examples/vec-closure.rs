pub trait UniformValue {
    fn set_uniform(&self, location: i32);
}

impl UniformValue for f32 {
    fn set_uniform(&self, location: i32) {
        println!("set uniform {}", location);
    }
}

fn main() {
    let mut v: Vec<Box<dyn Fn() -> Box<dyn UniformValue>>> = Vec::new();
    v.push(Box::new(|| Box::new(32.0)));

    v.get(0).unwrap()().set_uniform(4);
    // println!("foo {:?}", .);
}
