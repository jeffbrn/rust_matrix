#[cfg(test)]
use matrixlib::Matrix;

#[test]
fn new() {
    let mut m = Matrix::<i32,2,3>::new();
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
    assert_eq!(m[(0, 0)], 0);
    assert_eq!(m[(1, 0)], 0);
    assert_eq!(m[(0, 1)], 0);
    assert_eq!(m[(1, 1)], 0);
    assert_eq!(m[(0, 2)], 0);
    assert_eq!(m[(1, 2)], 0);
    m.set(&[11, 12, 13, 21, 22, 23]);
    println!("{:?}", m);
    assert_eq!(m[(0, 0)], 11);
    assert_eq!(m[(1, 0)], 21);
    assert_eq!(m[(0, 1)], 12);
    assert_eq!(m[(1, 1)], 22);
    assert_eq!(m[(0, 2)], 13);
    assert_eq!(m[(1, 2)], 23);
}

#[test]
fn test_default() {
    let m = Matrix::<f64,2,2>::default();
    assert_eq!(m[(0,0)], 0.0);
    assert_eq!(m[(0,1)], 0.0);
    assert_eq!(m[(1,0)], 0.0);
    assert_eq!(m[(1,1)], 0.0);
}

#[test]
fn indexing() {
	let mut m = Matrix::<f32,3,3>::new();
	m[(1,2)] = 123.4;
	m[(2,1)] = m[(1,2)] * 2.0;
	assert_eq!(m[(2,1)], 123.4*2.0);
    let expected = Matrix::<f32,3,3>::new_init(&[0.0, 0.0, 0.0, 0.0, 0.0, 123.4, 0.0, 246.8, 0.0]);
    assert_eq!(m, expected);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn index_fail() {
	let m = Matrix::<f32,3,3>::new();
    let _x = m[(3,3)];
}

#[test]
fn add_and_subtract() {
    let mut m1 = Matrix::<i32,3,2>::new();
    m1.set(&[11, 12, 21, 22, 31, 32]);
    println!("{:?}", m1);
    let mut m2 = Matrix::<i32,3,2>::new();
    m2.set(&[100, 200, 300, 400, 500, 600]);
    println!("{:?}", m2);
    let result1 = m1 + m2;
    println!("{:?}", result1);
    let expected_add = Matrix::<i32,3,2>::new_init(&[111, 212, 321, 422, 531, 632]);
    assert_eq!(result1, expected_add);
    let result2 = m2 - m1;
    println!("{:?}", result2);
    let expected_sub = Matrix::<i32,3,2>::new_init(&[89, 188, 279, 378, 469, 568]);
    assert_eq!(result2, expected_sub);
}

#[test]
fn identity() {
    let m2 = Matrix::<i32,2,3>::diagonal(1);
    let expected2 = Matrix::<i32,2,3>::new_init(&[1,0,0,0,1,0]);
    assert_eq!(m2, expected2);

    let m1 = Matrix::<i32,3,3>::diagonal(1);
    let expected1 = Matrix::<i32,3,3>::new_init(&[1,0,0,0,1,0,0,0,1]);
    assert_eq!(m1, expected1);

    let m3 = Matrix::<i32,3,2>::diagonal(1);
    let expected3 = Matrix::<i32,3,2>::new_init(&[1,0,0,1,0,0]);
    assert_eq!(m3, expected3);
}

#[test]
fn transpose() {
    let m1 = Matrix::<i32,3,2>::new_init(
        &[11, 12,
         21, 22,
         31, 32]);
    println!("{:?}", m1);
    let m2 = m1.transpose();
    assert_eq!(m2.rows(), 2);
    assert_eq!(m2.cols(), 3);
    let expected1 = Matrix::<i32,2,3>::new_init(
        &[11, 21, 31,
        12, 22, 32]);
    assert_eq!(m2, expected1);

    let m3 = Matrix::<u8,4,4>::diagonal(1);
    let m4 = m3.transpose();
    assert_eq!(m3, m4);

    let t1 = m1.transpose().transpose();
    assert_eq!(t1, m1);
}

#[test]
fn scalar_add_and_subtract() {
    let mut m1 = Matrix::<i32,3,2>::new();
    m1.set(&[11, 12, 21, 22, 31, 32]);
    println!("{:?}", m1);
    let result1 = m1 + 4;
    println!("{:?}", result1);
    let expected_add = Matrix::<i32,3,2>::new_init(&[15, 16, 25, 26, 35, 36]);
    assert_eq!(result1, expected_add);
    let result2 = m1 - 4;
    let expected_sub = Matrix::<i32,3,2>::new_init(&[7, 8, 17, 18, 27, 28]);
    assert_eq!(result2, expected_sub);
}

#[test]
fn scalar_mult() {
    let m = Matrix::<i32,3,2>::new_init(&[11, 12, 21, 22, 31, 32]);
    let result = m * 2;
    let expected = Matrix::<i32,3,2>::new_init(&[22, 24, 42, 44, 62, 64]);
    assert_eq!(result, expected);
}

#[test]
fn mult() {
    let m1 = Matrix::<i32,3,2>::new_init(&[3,4,-1,2,0,4]);
    let m2 = Matrix::<i32,2,2>::new_init(&[5,1,3,1]);
    let expected1 = Matrix::<i32,3,2>::new_init(&[27,7,1,1,12,4]);
    let result1 = m1 * m2;
    assert_eq!(result1, expected1);

    let m3 = Matrix::<i16,2,3>::new_init(&[3,0,3,1,1,0]);
    let m4 = Matrix::<i16,3,2>::new_init(&[4,4,0,1,4,1]);
    let expected2 = Matrix::<i16,2,2>::new_init(&[24,15,4,5]);
    let result2 = m3 * m4;
    assert_eq!(result2, expected2);

    let m5 = Matrix::<i16,2,3>::new_init(&[3,4,0,0,4,1]);
    let m6 = Matrix::<i16,3,3>::new_init(&[2,-1,2,4,0,0,0,1,1]);
    let expected3 = Matrix::<i16,2,3>::new_init(&[22,-3,6,16,1,1]);
    let result3 = m5 * m6;
    assert_eq!(result3, expected3);
}
