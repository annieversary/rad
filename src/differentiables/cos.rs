use crate::{
    differentiables::{mul::*, neg::*, sin::*, *},
    domain::Domain,
};
use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Cos<T, A: Differentiable<T>>(A, PhantomData<T>);
pub trait Cosable {
    fn cos(v: Self) -> Self;
}
impl<T, A> Differentiable<T> for Cos<T, A>
where
    T: Domain
        + Cosable
        + ops::Neg<Output = T>
        + Sinable
        + ops::Add<T, Output = T>
        + ops::Mul<T, Output = T>,
    A: Differentiable<T>,
{
    type Return = Mul<T, D<T, Neg<T, D<T, Sin<T, A>>>>, D<T, A::Return>>;

    fn calc(&self, v: T) -> T {
        T::cos(self.0.calc(v))
    }

    fn diff(&self) -> D<T, Self::Return> {
        mul(neg(sin(self.0.clone())), self.0.diff())
    }
}
pub fn cos<T, A>(a: A) -> D<T, Cos<T, A>>
where
    A: Differentiable<T>,
    T: Domain
        + Cosable
        + ops::Neg<Output = T>
        + Sinable
        + ops::Add<T, Output = T>
        + ops::Mul<T, Output = T>,
{
    D(Cos(a, PhantomData), PhantomData)
}
