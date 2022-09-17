use crate::{
    differentiables::{add::*, *},
    domain::Domain,
};
use std::ops;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Mul<T, A: Differentiable<T>, B: Differentiable<T>>(A, B, PhantomData<T>);
impl<T, A, B> Differentiable<T> for Mul<T, A, B>
where
    T: Domain,
    T: std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
    A: Differentiable<T>,
    B: Differentiable<T>,
{
    type Return = Add<T, D<T, Mul<T, D<T, A::Return>, B>>, D<T, Mul<T, A, D<T, B::Return>>>>;

    fn calc(&self, v: Values<T>) -> Result<T, VarNotProvided> {
        Ok(self.0.calc(v.clone())? * self.1.calc(v)?)
    }

    fn diff<ID: Var>(&self) -> D<T, Self::Return> {
        add(
            mul(self.0.diff::<ID>(), self.1.clone()),
            mul(self.0.clone(), self.1.diff::<ID>()),
        )
    }
}
pub fn mul<T, A, B>(a: A, b: B) -> D<T, Mul<T, A, B>>
where
    A: Differentiable<T>,
    B: Differentiable<T>,
    T: Domain,
    T: std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    D(Mul(a, b, PhantomData), PhantomData)
}

impl<T, A, B> ops::Mul<D<T, B>> for D<T, A>
where
    A: Differentiable<T>,
    B: Differentiable<T>,
    T: Domain,
    T: std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    type Output = D<T, Mul<T, D<T, A>, D<T, B>>>;

    fn mul(self, rhs: D<T, B>) -> Self::Output {
        mul(self, rhs)
    }
}

impl<T: Display, A: Differentiable<T> + Display, B: Differentiable<T> + Display> Display
    for Mul<T, A, B>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.0, self.1)
    }
}
