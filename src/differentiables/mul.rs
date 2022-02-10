use crate::{
    differentiables::{add::*, *},
    domain::Domain,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Mul<T, A: Differentiable<T>, B: Differentiable<T>>(A, B, PhantomData<T>);
impl<T, A, B> Differentiable<T> for Mul<T, A, B>
where
    T: Domain,
    T: std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
    A: Differentiable<T>,
    B: Differentiable<T>,
{
    type Return = Add<T, D<T, Mul<T, D<T, A::Return>, B>>, D<T, Mul<T, A, D<T, B::Return>>>>;

    fn calc(&self, v: T) -> T {
        self.0.calc(v.clone()) * self.1.calc(v)
    }

    fn diff(&self) -> D<T, Self::Return> {
        add(
            mul(self.0.diff(), self.1.clone()),
            mul(self.0.clone(), self.1.diff()),
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
