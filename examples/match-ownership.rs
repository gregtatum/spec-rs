struct Struct { pub value: i32 }

fn main() {
    let mut x = &Struct { value: 5 };

    match x {
        &ref x => { println!("Inside match {}", x.value); }
    };

    println!("Outside match {}", x.value);
}

/*
struct StructA {
    pub value: Option<StructB>
}

struct StructB {
    pub value: i32
}

fn main() {
    let mut x = StructA {
        value: Some(StructB {
            value: 5
        })
    };

    match x {
        StructA { value: Some(mut interior_value) } => {
            interior_value.value = 6;
            println!("Match {}", interior_value.value);
        },
        _ => {},
    };


    println!("Not match {}", x.value.unwrap().value);
}
*/
