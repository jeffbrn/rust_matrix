#[cfg(test)]
use matrixlib::{Vector, Matrix};

#[test]
fn test_new() {
    let mut v1 = Vector::<f32,3>::new();
    assert_eq!(v1.rows(), 3);
    assert_eq!(v1.cols(), 1);
    v1.set(&[1.0,2.0,3.0]);
    assert_eq!(v1[(0,0)], 1.0);
    assert_eq!(v1[(1,0)], 2.0);
    assert_eq!(v1[(2,0)], 3.0);

    let v2 = Vector::<f32,3>::new_init(&[1.0, 2.0, 3.0]);
    assert_eq!(v1, v2);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn index_fail() {
	let m = Vector::<f32,3>::new();
    let _x = m[(3,0)];
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
