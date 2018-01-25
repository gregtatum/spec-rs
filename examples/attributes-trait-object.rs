// Take a reference to some vector data, and pretend to transfer it over to the gl state.
trait BufferData { fn to_gl_buffer(&self) -> i32; }
impl BufferData for Vec<f32> { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 1 }}
impl BufferData for Vec<[f32; 2]> { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 2 }}
impl BufferData for Vec<[f32; 3]> { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 3 }}
impl BufferData for Vec<[f32; 4]> { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 4 }}
impl BufferData for Vec<[f32; 9]> { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 5 }}
impl BufferData for Vec<[f32; 16]> { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 6 }}
impl BufferData for i32 { fn to_gl_buffer(&self) -> i32 { println!("Convert and upload {:?}", *self); 7 }}

// Store a list of attributes for a draw command.
struct Attributes {
    pub list: Vec<(String, i32)>
}

// Add a "name" and "data" tuple for each attribute, this Attributes struct would be part of
// some larger struct.
impl Attributes {
    pub fn add(&mut self, name: String, data: &BufferData) {
        &self.list.push((name, data.to_gl_buffer()));
    }
}

fn main() {
    let mut attributes = Attributes { list: Vec::new() };
    let lambda = |num: i32| {
        num + 1
    };
    println!("{}", lambda(1));
    // Go ahead and pass by reference rather than box, because we don't need to retain this
    // data after we've transferred it to a gl buffer.
    attributes.add("points1d".to_string(), &vec![0.0, 1.0, 2.0]);
    attributes.add("points2d".to_string(), &vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]);
    attributes.add("points2d".to_string(), &5);

    for &(ref name, ref data) in attributes.list.iter() {
        println!("\nI sent this attribute to GL-land: {:?}, and got back {:?}", name, data);
    }
}
