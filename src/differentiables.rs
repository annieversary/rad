//! contains building blocks

use super::*;

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

#[derive(PartialEq, Debug, Clone)]
pub struct Neg<T, A: Differentiable<T>>(A, PhantomData<T>);
impl<T, A> Differentiable<T> for Neg<T, A>
where
    T: Domain + ops::Neg<Output = T>,
    A: Differentiable<T>,
{
    type Return = Neg<T, D<T, A::Return>>;

    fn calc(&self, v: T) -> T {
        ops::Neg::neg(self.0.calc(v))
    }

    fn diff(&self) -> D<T, Self::Return> {
        neg(self.0.diff())
    }
}
pub fn neg<T, A>(a: A) -> D<T, Neg<T, A>>
where
    A: Differentiable<T>,
    T: Domain + ops::Neg<Output = T>,
{
    D(Neg(a, PhantomData), PhantomData)
}
