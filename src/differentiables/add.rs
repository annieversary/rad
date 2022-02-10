use crate::{differentiables::*, domain::Domain};
use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Add<T, A: Differentiable<T>, B: Differentiable<T>>(A, B, PhantomData<T>);
impl<T, A, B> Differentiable<T> for Add<T, A, B>
where
    T: Domain,
    T: std::ops::Add<T, Output = T>,
    A: Differentiable<T>,
    B: Differentiable<T>,
{
    type Return = Add<T, D<T, A::Return>, D<T, B::Return>>;

    fn calc(&self, v: T) -> T {
        self.0.calc(v.clone()) + self.1.calc(v)
    }

    fn diff(&self) -> D<T, Self::Return> {
        add(self.0.diff(), self.1.diff())
    }
}
pub fn add<T, A, B>(a: A, b: B) -> D<T, Add<T, A, B>>
where
    A: Differentiable<T>,
    B: Differentiable<T>,
    T: Domain,
    T: std::ops::Add<T, Output = T>,
{
    D(Add(a, b, PhantomData), PhantomData)
}

impl<T, A, B> ops::Add<D<T, B>> for D<T, A>
where
    A: Differentiable<T>,
    B: Differentiable<T>,
    T: Domain,
    T: std::ops::Add<T, Output = T>,
{
    type Output = D<T, Add<T, D<T, A>, D<T, B>>>;

    fn add(self, rhs: D<T, B>) -> Self::Output {
        add(self, rhs)
    }
}
