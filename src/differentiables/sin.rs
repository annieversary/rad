use crate::{
    differentiables::{cos::*, mul::*, *},
    domain::Domain,
};
use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Sin<T, A: Differentiable<T>>(A, PhantomData<T>);
pub trait Sinable {
    fn sin(v: Self) -> Self;
}
impl<T, A> Differentiable<T> for Sin<T, A>
where
    T: Domain
        + Cosable
        + ops::Neg<Output = T>
        + Sinable
        + ops::Add<T, Output = T>
        + ops::Mul<T, Output = T>,
    A: Differentiable<T>,
{
    type Return = Mul<T, D<T, Cos<T, A>>, D<T, A::Return>>;

    fn calc(&self, v: T) -> T {
        T::sin(self.0.calc(v))
    }

    fn diff(&self) -> D<T, Self::Return> {
        mul(cos(self.0.clone()), self.0.diff())
    }
}
pub fn sin<T, A>(a: A) -> D<T, Sin<T, A>>
where
    A: Differentiable<T>,
    T: Domain
        + Cosable
        + ops::Neg<Output = T>
        + Sinable
        + ops::Add<T, Output = T>
        + ops::Mul<T, Output = T>,
{
    D(Sin(a, PhantomData), PhantomData)
}
