use crate::{consts::*, differentiables::*, domain::Domain};
use std::{fmt::Display, marker::PhantomData};

/// trait for variables
pub trait Var: 'static {}

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

    fn diff<ID2: Var>(&self) -> D<T, Self::Return> {
        use std::any::TypeId;

        if TypeId::of::<ID>() == TypeId::of::<ID2>() {
            c(T::ONE)
        } else {
            c(T::ZERO)
        }
    }
}

impl<T: 'static, ID: Var> Var for V<T, ID> {}
pub fn v<T: Domain, ID: Var + Clone>(_: ID) -> D<T, V<T, ID>> {
    D(V(PhantomData), PhantomData)
}

pub struct ConsID<A, B>(PhantomData<(A, B)>);

#[derive(PartialEq, Debug, Clone)]
pub struct X;
impl Var for X {}
#[derive(PartialEq, Debug, Clone)]
pub struct Y;
impl Var for Y {}

impl<T: Display> Display for V<T, X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X")
    }
}
impl<T: Display> Display for V<T, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Y")
    }
}
