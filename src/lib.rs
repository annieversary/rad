use std::marker::PhantomData;

/// the type that we differentiate over
/// will usually be a float or integer number
pub trait Domain: Clone {
    const ZERO: Self;
    const ONE: Self;
}

/// trait for differentiable things
pub trait Differentiable<T> {
    type Return: Differentiable<T>;

    fn calc(&self, v: T) -> T;
    fn diff(&self) -> D<T, Self::Return>;
}

/// wrapper type for differentiable things
#[derive(PartialEq, Debug, Clone)]
pub struct D<T, I: Differentiable<T>>(I, PhantomData<T>);
// pass calls to inner impl
impl<T, I: Differentiable<T>> Differentiable<T> for D<T, I> {
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
pub struct V<T, ID: Var>(PhantomData<(T, ID)>);
impl<T: Domain, ID: Var> Differentiable<T> for V<T, ID> {
    type Return = differentiables::Const<T>;

    fn calc(&self, v: T) -> T {
        v
    }

    fn diff(&self) -> D<T, Self::Return> {
        differentiables::c(T::ONE)
    }
}
impl<T, ID: Var> Var for V<T, ID> {}
pub fn v<T, ID: Var>(_v: ID) -> V<T, ID> {
    V(PhantomData)
}

pub mod differentiables {
    //! contains building blocks

    use super::*;

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
}

#[cfg(test)]
mod tests {
    use super::differentiables::*;
    use super::*;

    #[test]
    fn it_works() {
        struct X;
        impl Var for X {}

        impl Domain for i32 {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }

        let addition = add(c(1), v(X));
        let r = addition.diff();

        assert_eq!(r, add(c(0), c(1)));
        assert_eq!(r.calc(0), 1);
    }
}
