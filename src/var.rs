use crate::{consts::*, differentiables::*, domain::Domain};
use std::marker::PhantomData;

/// trait for variables
pub trait Var {}

/// wrapper type for variables
#[derive(PartialEq, Debug, Clone)]
pub struct V<T, ID: Var>(PhantomData<(T, ID)>);
impl<T: Domain, ID: Var> Differentiable<T> for V<T, ID>
where
    Self: Clone,
{
    type Return = Const<T>;

    fn calc(&self, v: T) -> T {
        v
    }

    fn diff(&self) -> D<T, Self::Return> {
        c(T::ONE)
    }
}
impl<T, ID: Var> Var for V<T, ID> {}
pub fn v<T, ID: Var>(_v: ID) -> V<T, ID> {
    V(PhantomData)
}

#[derive(PartialEq, Debug, Clone)]
pub struct X;
impl Var for X {}
#[derive(PartialEq, Debug, Clone)]
pub struct Y;
impl Var for Y {}
