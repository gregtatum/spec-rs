use std::mem;
use std::slice;

// Take a reference to some vector data, and pretend to transfer it over to the gl state.
pub trait BufferableData { fn to_buffer(&self) -> i32; }

fn create_buffer(data: &[f32]) -> i32 {
    println!("Uploading data to a gl buffer, beep boop {:?}", data);
    0
}

impl BufferableData for Vec<f32> {
    fn to_buffer(&self) -> i32 {
        create_buffer(self)
    }
}

/**
 * For all the rest of the code, we have to unsafely transmute our Vec<[f32; N]>
 * from a Vector, to a reference to slice. The actual data that the Vec points
 * to in the heap is correctly laid out how we want to use it on the GL side.
 * The basic process is to pull a pointer to our data, unsafely transmute_copy
 * a pointer to it, removing the type from the compiler. Finally we pass that
 * into the foreign function interface to be consumed by the gl state machine.
 */

impl BufferableData for Vec<[f32; 2]> {
    fn to_buffer(&self) -> i32 {
        unsafe {
            let data: &[f32] = slice::from_raw_parts(
                mem::transmute_copy::<*const [f32; 2], *mut f32>(&self.as_ptr()),
                self.len() * 2
            );
            create_buffer(data)
        }
    }
}

impl BufferableData for Vec<[f32; 3]> {
    fn to_buffer(&self) -> i32 {
        unsafe {
            let data: &[f32] = slice::from_raw_parts(
                mem::transmute_copy::<*const [f32; 3], *mut f32>(&self.as_ptr()),
                self.len() * 3
            );
            create_buffer(data)
        }
    }
}

pub fn main() {
    vec![0.0, 1.0, 2.0].to_buffer();
    vec![[0.0, 1.0], [2.0, 3.0]].to_buffer();
    vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]].to_buffer();
}
