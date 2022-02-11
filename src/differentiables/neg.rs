use crate::{differentiables::*, domain::Domain};
use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Neg<T, A: Differentiable<T>>(A, PhantomData<T>);
impl<T, A> Differentiable<T> for Neg<T, A>
where
    T: Domain + ops::Neg<Output = T>,
    A: Differentiable<T>,
{
    type Return = Neg<T, D<T, A::Return>>;

    fn calc(&self, v: Values<T>) -> Result<T, VarNotProvided> {
        self.0.calc(v).map(ops::Neg::neg)
    }

    fn diff<ID: Var>(&self) -> D<T, Self::Return> {
        neg(self.0.diff::<ID>())
    }
}
pub fn neg<T, A>(a: A) -> D<T, Neg<T, A>>
where
    A: Differentiable<T>,
    T: Domain + ops::Neg<Output = T>,
{
    D(Neg(a, PhantomData), PhantomData)
}

impl<T, A> ops::Neg for D<T, A>
where
    A: Differentiable<T>,
    T: Domain + ops::Neg<Output = T>,
{
    type Output = D<T, Neg<T, D<T, A>>>;

    fn neg(self) -> Self::Output {
        neg(self)
    }
}

impl<T: Display, A: Display + Differentiable<T>> Display for Neg<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-{}", self.0)
    }
}
