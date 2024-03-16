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
