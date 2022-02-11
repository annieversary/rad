use crate::{consts::*, differentiables::*, domain::Domain};
use std::any::TypeId;
use std::sync::Arc;
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

    fn calc(&self, vals: Values<T>) -> Result<T, VarNotProvided> {
        for (i, v) in &*vals.0 {
            if *i == TypeId::of::<ID>() {
                return Ok(v.clone());
            }
        }
        Err(VarNotProvided(std::any::type_name::<ID>()))
    }

    fn diff<ID2: Var>(&self) -> D<T, Self::Return> {
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

#[derive(Clone)]
pub struct Values<T>(Arc<[(TypeId, T)]>);
pub struct ValuesBuilder<T>(Vec<(TypeId, T)>);
impl<T> ValuesBuilder<T> {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn add<ID: Var>(mut self, _: ID, t: T) -> Self {
        self.0.push((TypeId::of::<ID>(), t));
        self
    }
    pub fn build(self) -> Values<T> {
        Values(self.0.into())
    }
}
/// Return a fully built `Values<T>` containing the single variable
pub fn val<T, ID: Var>(i: ID, v: T) -> Values<T> {
    ValuesBuilder::new().add(i, v).build()
}
/// Return a `ValuesBuilder<T>` containing the provided variable
pub fn vals<T, ID: Var>(i: ID, v: T) -> ValuesBuilder<T> {
    ValuesBuilder::new().add(i, v)
}

#[derive(PartialEq, Debug, Clone)]
pub struct X;
impl Var for X {}
#[derive(PartialEq, Debug, Clone)]
pub struct Y;
impl Var for Y {}
#[derive(PartialEq, Debug, Clone)]
pub struct Z;
impl Var for Z {}

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
impl<T: Display> Display for V<T, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Z")
    }
}
