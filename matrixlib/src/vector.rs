use super::Matrix;
use num_traits::Num;
use std::ops::Mul;

pub type Vector<T,const R:usize> = Matrix<T,R,1>;

impl<T: Num+Default+Copy+Mul, const R: usize> Vector<T,R> {
    pub fn dot(&self, other: &Vector<T,R>) -> T {
        if self.size.1 > 1 { panic!("only valid for vectors"); }
        let mut sum = T::default();
        for i in 0..R {
            let x = self.vals[i][0] * other.vals[i][0];
            sum = sum + x;
        }
        sum
    }
}
impl<T: Num+Default+Copy+Mul> Vector<T,2> {
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
}
impl<T: Num+Default+Copy+Mul> Vector<T,3> {
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn z(&self) -> T {
        self[2]
    }
}

impl<T: Num, const R: usize> std::ops::Index<usize> for Vector<T,R> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index][0]
    }
}
impl<T: Num, const R: usize> std::ops::IndexMut<usize> for Vector<T,R> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vals[index][0]
    }
}

impl<T: Num+Default+Copy+Mul> Mul for Vector<T,3> {
    type Output = Vector<T,3>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector::<T,3>::new_init(&[self.y()*rhs.z()-self.z()*rhs.y(),self.z()*rhs.x()-self.x()*rhs.z(),self.x()*rhs.y()-self.y()*rhs.x()])
    }
}