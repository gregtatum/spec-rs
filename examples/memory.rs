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

#[allow(dead_code)]
struct MyStruct {
    a: i32,
    b: i32
}

fn main() {
    let array: [i32; 4] = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
    let slice: &[i32] = &array[..];
    let mut vector: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    vector.push(0);
    let my_box: Box<[i32; 4]> = Box::new([0x11111111, 0x22222222, 0x33333333, 0x44444444]);
    let fat_box: Box<[i32]> = (vec![0x11111111, 0x22222222, 0x33333333]).into_boxed_slice();
    let my_struct = MyStruct {a: 0x11111111, b: 0x22222222};


    unsafe {
        output_size("array    ", &array);   // | i32 | i32 | i32 | i32 |
        let array_stack: [u8; 16] = mem::transmute_copy(&array);
        println!("(stack)   | i32         | i32         | i32         | i32         |");
        println!("          {:x}|\n", ByteBuf(&array_stack, 4));

        output_size("slice    ", &slice);   // | pointer | length |
        let slice_values: [u8; 16] = mem::transmute_copy(&slice);
        println!("(stack)   | pointer                 | length                  |");
        println!("          {:x}|\n", ByteBuf(&slice_values, 8));

        output_size("vector   ", &vector); // | pointer | capacity | length |
        let vector_values: [u8; 24] = mem::transmute_copy(&vector);
        println!("(stack)   | pointer                 | capacity                | length                  |");
        println!("          {:x}|\n", ByteBuf(&vector_values, 8));

        output_size("my_box   ", &my_box);    //
        let my_box_values: [u8; 8] = mem::transmute_copy(&my_box);
        println!("(stack)   | pointer                 |");
        println!("          {:x}|\n", ByteBuf(&my_box_values, 8));

        output_size("*my_box  ", &*my_box);
        let inside_my_box_values: [u8; 16] = mem::transmute_copy(&*my_box);
        println!("(heap)    | i32         | i32         | i32         | i32         |");
        println!("          {:x}|\n", ByteBuf(&inside_my_box_values, 4));

        output_size("fat_box  ", &fat_box);
        let fat_box_values: [u8; 16] = mem::transmute_copy(&fat_box);
        println!("(stack)   | pointer                 | length                  |");
        println!("          {:x}|\n", ByteBuf(&fat_box_values, 8));

        output_size("my_struct", &my_struct);
        let my_struct_values: [u8; 8] = mem::transmute_copy(&my_struct);
        println!("(stack)   | i32         | i32         |");
        println!("          {:x}|\n", ByteBuf(&my_struct_values, 4));


    }
}

/*
-----------------------------------------------------------------------------------------
array     ( 16 bytes, 128 bits, 2 words )
(stack)   | i32         | i32         | i32         | i32         |
          | 11 11 11 11 | 22 22 22 22 | 33 33 33 33 | 44 44 44 44 |
-----------------------------------------------------------------------------------------
slice     ( 16 bytes, 128 bits, 2 words )
(stack)   | pointer                 | length                  |
          | 88 bd 7c 52 ff 7f 00 00 | 04 00 00 00 00 00 00 00 |
-----------------------------------------------------------------------------------------
vector    ( 24 bytes, 192 bits, 3 words )
(stack)   | pointer                 | capacity                | length                  |
          | 50 b0 81 0d 01 00 00 00 | 12 00 00 00 00 00 00 00 | 0a 00 00 00 00 00 00 00 |
-----------------------------------------------------------------------------------------
my_box    ( 8 bytes, 64 bits, 1 words )
(stack)   | pointer                 |
          | 00 00 82 0d 01 00 00 00 |
-----------------------------------------------------------------------------------------
*my_box   ( 16 bytes, 128 bits, 2 words )
(heap)    | i32         | i32         | i32         | i32         |
          | 11 11 11 11 | 22 22 22 22 | 33 33 33 33 | 44 44 44 44 |
-----------------------------------------------------------------------------------------
fat_box   ( 16 bytes, 128 bits, 2 words )
(stack)   | pointer                 | length                  |
          | 10 00 82 0d 01 00 00 00 | 03 00 00 00 00 00 00 00 |
-----------------------------------------------------------------------------------------
my_struct ( 8 bytes, 64 bits, 1 words )
(stack)   | i32         | i32         |
          | 11 11 11 11 | 22 22 22 22 |
*/
