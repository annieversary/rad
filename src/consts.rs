use crate::differentiables::Differentiable;
use crate::domain::Domain;
use crate::var::{Values, Var};
use std::marker::PhantomData;

use crate::differentiables::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Const<T>(pub T);
impl<T: Domain> Differentiable<T> for Const<T> {
    type Return = Const<T>;

    fn calc(&self, _: Values<T>) -> Result<T, VarNotProvided> {
        Ok(self.0.clone())
    }

    fn diff<ID: Var>(&self) -> D<T, Self::Return> {
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

impl<T: std::fmt::Display> std::fmt::Display for Const<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
