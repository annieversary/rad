pub mod consts;
pub mod differentiables;
pub mod domain;
pub mod var;

// TODO implement ops for the stuff, so we can do `c(1) + sin(c(1))`

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::differentiables::{add::*, cos::*, mul::*, neg::*, sin::*, *};
    pub use crate::domain::*;
    pub use crate::var::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

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

        assert_eq!(r, c(0) + c(1));
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
        let cos = cos(v(X)) + cos(v(X));

        assert_eq!(cos.calc(0.0), 2.0);
    }

    #[test]
    fn calc_mul_cos() {
        let cos = mul(cos(v(X)), c(1.0));

        assert_eq!(cos.calc(0.0), 1.0);
    }

    #[test]
    fn addition_with_sin() {
        let addition = c(1.0) + sin(v(X));
        assert_eq!(addition.calc(0.0), 1.0);

        let r = addition.diff();

        assert_eq!(r, c(0.0) + mul(cos(v(X)), c(1.0)));
        assert_eq!(r.calc(0.0), 1.0);
    }

    #[test]
    fn weird_one() {
        let v = cos(c(1.0) + sin(v(X)));

        let r = v.diff();

        dbg!(r);
        // assert_eq!(dbg!(r), mul(sin(add(c(1.0), sin(v(X)))), neg(cos(v(X)))));
    }
}
