#[cfg(test)]
use matrixlib::Matrix;

#[test]
fn create() {
    let mut m = Matrix::<i32,2,3>::new();
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
    assert_eq!(m[(0, 0)], 0);
    assert_eq!(m[(1, 0)], 0);
    assert_eq!(m[(0, 1)], 0);
    assert_eq!(m[(1, 1)], 0);
    assert_eq!(m[(0, 2)], 0);
    assert_eq!(m[(1, 2)], 0);
    m.init(&[11, 12, 13, 21, 22, 23]);
    println!("{:?}", m);
    assert_eq!(m[(0, 0)], 11);
    assert_eq!(m[(1, 0)], 21);
    assert_eq!(m[(0, 1)], 12);
    assert_eq!(m[(1, 1)], 22);
    assert_eq!(m[(0, 2)], 13);
    assert_eq!(m[(1, 2)], 23);
}

#[test]
fn add_and_subtract() {
    let mut m1 = Matrix::<i32,3,2>::new();
    m1.init(&[11, 12, 21, 22, 31, 32]);
    println!("{:?}", m1);
    let mut m2 = Matrix::<i32,3,2>::new();
    m2.init(&[100, 200, 300, 400, 500, 600]);
    println!("{:?}", m2);
    let result1 = m1 + m2;
    println!("{:?}", result1);
    assert_eq!(result1[(0, 0)], 111);
    assert_eq!(result1[(0, 1)], 212);
    assert_eq!(result1[(1, 0)], 321);
    assert_eq!(result1[(1, 1)], 422);
    assert_eq!(result1[(2, 0)], 531);
    assert_eq!(result1[(2, 1)], 632);
    let result2 = m2 - m1;
    println!("{:?}", result2);
    assert_eq!(result2[(0, 0)], 89);
    assert_eq!(result2[(0, 1)], 188);
    assert_eq!(result2[(1, 0)], 279);
    assert_eq!(result2[(1, 1)], 378);
    assert_eq!(result2[(2, 0)], 469);
    assert_eq!(result2[(2, 1)], 568);
}

