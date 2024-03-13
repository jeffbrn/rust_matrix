use std::usize;
use num_traits::Num;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Matrix<T, const R: usize, const C: usize> where T : Num {
    vals: [[T; C]; R],
    size: (usize, usize),
}
pub type Vector<T,const R:usize> = Matrix<T,R,1>;

impl<T: Num+Default+Copy, const R: usize, const C: usize> Matrix<T,R,C> {
    pub fn new() -> Matrix<T,R,C> {
        let m: Matrix<T,R,C> = Matrix {
            vals: [[T::default(); C]; R],
            size: (R, C)
        };
        m
    }

    pub fn init(&mut self, vals_: &[T]) {
        let n = R * C;
        assert!(vals_.len() <= n);
        for idx in 0..vals_.len() {
            let i = idx / self.size.1;
            let j = idx % self.size.1;
            self.vals[i][j] = vals_[idx];
        }
    }
}

impl<T: Num, const R: usize, const C: usize> std::ops::Index<(usize, usize)> for Matrix<T,R,C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vals[index.0][index.1]
    }
}
impl<T: Num, const R: usize, const C: usize> std::ops::IndexMut<(usize, usize)> for Matrix<T,R,C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vals[index.0][index.1]
    }
}
impl<T: Num+Default+Copy, const R: usize, const C: usize> std::ops::Add for Matrix<T,R,C> {
    type Output = Matrix<T,R,C>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::<T,R,C>::new();
        for i in 0..R {
            for j in 0..C  {
                result[(i,j)] = self[(i,j)] + rhs[(i,j)];
            }
        }
        result
    }
}
impl<T: Num+Default+Copy, const R: usize, const C: usize> std::ops::Sub for Matrix<T,R,C> {
    type Output = Matrix<T,R,C>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::<T,R,C>::new();
        for i in 0..R {
            for j in 0..C  {
                result[(i,j)] = self[(i,j)] - rhs[(i,j)];
            }
        }
        result
    }
}

#[test]
fn test_init() {
    let mut m = Matrix::<i32,2,3>::new();
    assert_eq!(m.size.0, 2);
    assert_eq!(m.size.1, 3);
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
fn test_add_sub() {
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

#[test]
fn test_vector() {
    let mut v1 = Vector::<f32,3>::new();
    assert_eq!(v1.size.0, 3);
    assert_eq!(v1.size.1, 1);
    v1.init(&[1.0,2.0,3.0]);
    assert_eq!(v1[(0,0)], 1.0);
    assert_eq!(v1[(1,0)], 2.0);
    assert_eq!(v1[(2,0)], 3.0);
}
