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

    /// calculate the value for this expression with the provided values
    fn calc(&self, v: Values<T>) -> Result<T, VarNotProvided>;

    /// shorthand for calculating an expression that only contains `X`
    fn calc_x(&self, x: T) -> Result<T, VarNotProvided> {
        Self::calc(self, val(X, x))
    }
    /// shorthand for calculating an expression that only contains `X` and `Y`
    fn calc_xy(&self, x: T, y: T) -> Result<T, VarNotProvided> {
        Self::calc(self, vals(X, x).add(Y, y).build())
    }
    /// shorthand for calculating an expression that only contains `X`, `Y`, and `Z`
    fn calc_xyz(&self, x: T, y: T, z: T) -> Result<T, VarNotProvided> {
        Self::calc(self, vals(X, x).add(Y, y).add(Z, z).build())
    }

    /// differentiate this expression in respect of the `ID` variable
    fn diff<ID: Var>(&self) -> D<T, Self::Return>;
}

/// wrapper type for differentiable things
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct D<T, I: Differentiable<T>>(pub(crate) I, pub(crate) PhantomData<T>);
// pass calls to inner impl
impl<T, I: Differentiable<T>> Differentiable<T> for D<T, I>
where
    Self: Clone,
{
    type Return = I::Return;

    fn calc(&self, v: Values<T>) -> Result<T, VarNotProvided> {
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

#[derive(Debug, PartialEq, Eq)]
pub struct VarNotProvided(pub &'static str);
