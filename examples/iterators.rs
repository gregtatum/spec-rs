fn main() {
    let mut vec = (0..100).map(|i| {
        let fi = i as f32;
        [fi, fi, fi]
    }).collect::<Vec<[f32; 3]>>();

    vec = vec as Vec<f32>;
    println!("{:?}", vec);
}
