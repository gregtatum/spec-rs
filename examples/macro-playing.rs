macro_rules! clear {
    ($($key:ident => $value:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $( clear_key!(temp_vec, $key => $value); )*
            temp_vec
        }
    };
}

macro_rules! clear_key {
    ($temp_vec:expr, color => $value:expr) => {
        match &mut $temp_vec {
            vec => {
                vec.push("color");
                vec.push($value);
            }
        };
    };
    ($temp_vec:expr, depth => $value:expr) => {
        match &mut $temp_vec {
            vec => {
                vec.push("depth");
                vec.push($value);
            }
        };
    };
}

fn main() {
    let clear_red = clear![
        color => "red",
        depth => "0"
    ];
    let clear_blue = clear![color => "blue"];

    println!("{:?}", clear_red);
    println!("{:?}", clear_blue);
}
