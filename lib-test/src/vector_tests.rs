#[cfg(test)]
use matrixlib::Vector;

#[test]
fn test_new() {
    let mut v1 = Vector::<f32,3>::new();
    assert_eq!(v1.rows(), 3);
    assert_eq!(v1.cols(), 1);
    v1.init(&[1.0,2.0,3.0]);
    assert_eq!(v1[(0,0)], 1.0);
    assert_eq!(v1[(1,0)], 2.0);
    assert_eq!(v1[(2,0)], 3.0);

    let v2 = Vector::<f32,3>::new_init(&[1.0, 2.0, 3.0]);
    assert_eq!(v1, v2);
}
