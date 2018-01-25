// rustc --pretty expanded ./examples/macro-expansion

macro_rules! cb {
    ( $x:expr, $y:expr ) => {
        {
            let mut temp_vec = Vec::new();
            temp_vec.push($x);
            temp_vec.push($y);
            temp_vec
        }
    };
}



fn main() {
    cb![Box:new(0), Box::new(1)]
}
