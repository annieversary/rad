use crate::differentiables::Differentiable;
use crate::domain::Domain;
use std::marker::PhantomData;

use crate::differentiables::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Const<T>(pub T);
impl<T: Domain> Differentiable<T> for Const<T> {
    type Return = Const<T>;

    fn calc(&self, _: T) -> T {
        self.0.clone()
    }

    fn diff(&self) -> D<T, Self::Return> {
        c(T::ZERO)
    }
}
impl<T: Domain> From<T> for D<T, Const<T>> {
    fn from(t: T) -> Self {
        D(Const(t), PhantomData)
    }
}
pub fn c<T: Domain>(t: T) -> D<T, Const<T>> {
    t.into()
}
