extern crate rand;
use std::mem;

struct ByteBuf<'a> (&'a [u8], i32);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut i = 0;
        for byte in self.0 {
            if i % self.1 == 0 {
                fmtr.write_fmt(format_args!("| "))?;
            }
            fmtr.write_fmt(format_args!("{:02x} ", byte))?;
            i += 1;
        }
        Ok(())
    }
}

fn output_size<T>(name: &'static str, value: &T) {
    let size = mem::size_of_val(value);
    println!("-----------------------------------------------------------------------------------------");
    println!("{} ( {} bytes, {} bits, {} words )",
        name,
        size,
        8 * size,
        size as f64 / mem::size_of::<usize>() as f64
    );
}

fn main() {
    let vecs1 = vec![0.0, 0.0, 1.0, 1.0];

    unsafe {
        output_size("vecs1   ", &vecs1);    //
        let my_box_values: [u8; 24] = mem::transmute_copy(&vecs1);
        // println!("(stack)   | pointer                 |");
        println!("          {:x}|\n", ByteBuf(&my_box_values, 8));

        // output_size("*vecs1  ", &*vecs1);
        // let inside_my_box_values: [u8; 16] = mem::transmute_copy(&*vecs1);
        // println!("(heap)    | i32         | i32         | i32         | i32         |");
        // println!("          {:x}|\n", ByteBuf(&inside_my_box_values, 4));
    }
}
