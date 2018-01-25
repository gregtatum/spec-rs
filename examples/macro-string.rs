macro_rules! to_string {
    (bar) => {to_string!()};
    (foo) => {to_string!()};
    () => {"returned value"};
}

fn main() {
    let foo: &str = to_string!(foo);
    println!("{:?}", foo);
}
