use crate::{
    differentiables::{mul::*, neg::*, sin::*, *},
    domain::Domain,
};
use std::ops;

#[derive(PartialEq, Eq, Debug, Clone)]
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

    fn calc(&self, v: Values<T>) -> Result<T, VarNotProvided> {
        self.0.calc(v).map(T::cos)
    }

    fn diff<ID: Var>(&self) -> D<T, Self::Return> {
        mul(neg(sin(self.0.clone())), self.0.diff::<ID>())
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

impl<T: Display, A: Display + Differentiable<T>> Display for Cos<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cos({})", self.0)
    }
}
