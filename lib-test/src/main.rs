mod vector_tests;
mod matrix_tests;

use matrixlib::vector::Vector;

fn main() {
    println!("Hello, world!");
    let mut v = Vector::<f32,3>::new();
    v.set(&[1.0, 2.0, 3.0]);
    println!("{:?}", v);
}
