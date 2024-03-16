#[cfg(test)]
use matrixlib::{vector::Vector, Matrix};

#[test]
fn test_new() {
    let mut v1 = Vector::<f32,3>::new();
    assert_eq!(v1.rows(), 3);
    assert_eq!(v1.cols(), 1);
    v1.set(&[1.0,2.0,3.0]);
    assert_eq!(v1[0], 1.0);
    assert_eq!(v1[1], 2.0);
    assert_eq!(v1[2], 3.0);
    assert_eq!(v1.x(), 1.0);
    assert_eq!(v1.y(), 2.0);
    assert_eq!(v1.z(), 3.0);

    let v2 = Vector::<f32,3>::new_init(&[1.0, 2.0, 3.0]);
    assert_eq!(v1, v2);
}

#[test]
fn test_vec2() {
    let v = Vector::<i32,2>::new_init(&[11,12]);
    assert_eq!(v.x(), 11);
    assert_eq!(v.y(), 12);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn index_fail() {
	let m = Vector::<f32,3>::new();
    let _x = m[3];
}

#[test]
fn transpose() {
    let v = Vector::<f32,3>::new_init(&[1.0, 2.0, 3.0]);
    let t = v.transpose();
    assert_eq!(t.rows(), 1);
    assert_eq!(t.cols(), 3);
    let expected = Matrix::<f32,1,3>::new_init(&[1.0, 2.0, 3.0]);
    assert_eq!(t, expected);

    let x = v.transpose().transpose();
    assert_eq!(x, v);
}

#[test]
fn dot_product() {
    let data = [2, 3, 4];
    let v = Vector::<i32,3>::new_init(&data);
    let dp = v.dot(&v);
    assert_eq!(dp, 29);
}

#[test]
fn cross_product() {
    let v11 = Vector::<i32,3>::new_init(&[5,3,4]);
    let v12 = Vector::<i32,3>::new_init(&[-2,1,-1]);
    let expected1 = Vector::<i32,3>::new_init(&[-7,-3,11]);
    let result1 = v11 * v12;
    assert_eq!(result1, expected1);

    let v21 = Vector::<i32,3>::new_init(&[1,1,1]);
    let v22 = Vector::<i32,3>::new_init(&[2,2,2]);
    let expected2 = Vector::<i32,3>::new();
    let result2 = v21 * v22;
    assert_eq!(result2, expected2);

    let v31 = Vector::<i32,3>::new_init(&[1,0,0]);
    let v32 = Vector::<i32,3>::new_init(&[0,1,0]);
    let expected3 = Vector::<i32,3>::new_init(&[0,0,1]);
    let result3 = v31 * v32;
    assert_eq!(result3, expected3);
}
