use matrixlib::Vector;

fn main() {
    println!("Hello, world!");
    let mut v = Vector::<f32,3>::new();
    v.init(&[1.0, 2.0, 3.0]);
    println!("{:?}", v);
}
