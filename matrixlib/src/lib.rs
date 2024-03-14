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
    pub fn new_init(vals_: &[T]) -> Matrix<T,R,C> {
        let n = R * C;
        assert!(vals_.len() <= n);
        let mut data = [[T::default(); C]; R];
        for (idx, val) in vals_.iter().enumerate() {
            let i = idx / C;
            let j = idx % C;
            data[i][j] = *val;
        }
        let m: Matrix<T,R,C> = Matrix {
            vals: data,
            size: (R, C)
        };
        m
    }
    pub fn diagonal(val: T) -> Matrix<T,R,C> {
        let n = std::cmp::min(R, C);
        let mut data = [[T::default(); C]; R];
        for i in 0..n {
            data[i][i] = val;
        }
        Matrix {
            vals: data,
            size: (R, C)
        }
    }

    pub fn init(&mut self, vals_: &[T]) {
        let n = R * C;
        assert!(vals_.len() <= n);
        for (idx, val) in vals_.iter().enumerate() {
            let i = idx / self.size.1;
            let j = idx % self.size.1;
            self.vals[i][j] = *val;
        }
    }
    pub fn rows(&self) -> usize {
        self.size.0
    }
    pub fn cols(&self) -> usize {
        self.size.1
    }
}
impl<T: Num+Default+Copy, const R: usize, const C: usize> Default for Matrix<T,R,C> {
    fn default() -> Self {
        Self { vals: [[T::default(); C]; R], size: (R, C) }
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
    for i in 0..m.rows() {
        for j in 0..m.cols() {
            assert_eq!(m.vals[i][j], 0);
        }
    }
    m.init(&[11, 12, 13, 21, 22, 23]);
    println!("{:?}", m);
    for i in 0..m.rows() {
        for j in 0..m.cols() {
            let expected = (i+1)*10+(j+1);
            assert_eq!(m.vals[i][j], expected as i32);
        }
    }

    let n = Matrix::<i32,2,3>::new_init(&[11, 12, 13, 21, 22, 23]);
    for i in 0..n.rows() {
        for j in 0..n.cols() {
            let expected = (i+1)*10+(j+1);
            assert_eq!(n.vals[i][j], expected as i32);
        }
    }

    assert_eq!(m, n);
}
