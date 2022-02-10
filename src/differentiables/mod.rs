//! contains building blocks

pub mod add;
pub mod cos;
pub mod mul;
pub mod neg;
pub mod sin;

use crate::var::*;
use std::{fmt::Display, marker::PhantomData};

/// trait for differentiable things
pub trait Differentiable<T>: Clone {
    type Return: Differentiable<T>;

    fn calc(&self, v: T) -> T;
    fn diff<ID: Var>(&self) -> D<T, Self::Return>;
}

/// wrapper type for differentiable things
#[derive(PartialEq, Debug, Clone)]
pub struct D<T, I: Differentiable<T>>(pub(crate) I, pub(crate) PhantomData<T>);
// pass calls to inner impl
impl<T, I: Differentiable<T>> Differentiable<T> for D<T, I>
where
    Self: Clone,
{
    type Return = I::Return;

    fn calc(&self, v: T) -> T {
        I::calc(&self.0, v)
    }

    fn diff<ID: Var>(&self) -> D<T, Self::Return> {
        I::diff::<ID>(&self.0)
    }
}

impl<T: Display, I: Display + Differentiable<T>> Display for D<T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
