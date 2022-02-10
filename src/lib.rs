use std::marker::PhantomData;

pub mod differentiables;

/// the type that we differentiate over
/// will usually be a float or integer number
pub trait Domain: Clone {
    const ZERO: Self;
    const ONE: Self;
}

/// trait for differentiable things
pub trait Differentiable<T>: Clone {
    type Return: Differentiable<T>;

    fn calc(&self, v: T) -> T;
    fn diff(&self) -> D<T, Self::Return>;
}

/// wrapper type for differentiable things
#[derive(PartialEq, Debug, Clone)]
pub struct D<T, I: Differentiable<T>>(I, PhantomData<T>);
// pass calls to inner impl
impl<T, I: Differentiable<T>> Differentiable<T> for D<T, I>
where
    Self: Clone,
{
    type Return = I::Return;

    fn calc(&self, v: T) -> T {
        I::calc(&self.0, v)
    }

    fn diff(&self) -> D<T, Self::Return> {
        I::diff(&self.0)
    }
}

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

#[cfg(test)]
mod tests {
    use super::differentiables::*;
    use super::*;

    // TODO move this to inner code
    #[derive(PartialEq, Debug, Clone)]
    struct X;
    impl Var for X {}

    // TODO move this impl to another place
    impl Domain for f32 {
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
    }
    impl Sinable for f32 {
        fn sin(v: Self) -> Self {
            v.sin()
        }
    }
    impl Cosable for f32 {
        fn cos(v: Self) -> Self {
            v.cos()
        }
    }
    impl Domain for i32 {
        const ZERO: Self = 0;
        const ONE: Self = 1;
    }

    #[test]
    fn simple_addition() {
        let addition = add(c(1), v(X));
        let r = addition.diff();

        assert_eq!(r, add(c(0), c(1)));
        // calc won't use the value passed in, since `r` is a constant, but we still need to pass a parameter
        assert_eq!(r.calc(0), 1);
    }

    #[test]
    fn calc_val() {
        let v = v(X);

        assert_eq!(v.calc(33.0), 33.0);
    }

    #[test]
    fn calc_add_cos() {
        let cos = add(cos(v(X)), cos(v(X)));

        assert_eq!(cos.calc(0.0), 2.0);
    }

    #[test]
    fn calc_mul_cos() {
        let cos = mul(cos(v(X)), c(1.0));

        assert_eq!(cos.calc(0.0), 1.0);
    }

    #[test]
    fn addition_with_sin() {
        let addition = add(c(1.0), sin(v(X)));
        assert_eq!(addition.calc(0.0), 1.0);

        let r = addition.diff();

        assert_eq!(r, add(c(0.0), mul(cos(v(X)), c(1.0))));
        assert_eq!(r.calc(0.0), 1.0);
    }
}
