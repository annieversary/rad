pub mod consts;
pub mod default_impls;
pub mod differentiables;
pub mod domain;
pub mod var;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::default_impls::*;
    pub use crate::differentiables::{add::*, cos::*, mul::*, neg::*, sin::*, *};
    pub use crate::domain::*;
    pub use crate::var::*;
}

#[cfg(test)]
mod single_var_tests {
    use super::prelude::*;

    #[test]
    fn simple_addition() {
        let addition = v(X) + c(1i32);
        let r = addition.diff::<X>();

        assert_eq!(r, c(1) + c(0));
        // calc won't use the value passed in, since `r` is a constant, but we still need to pass a parameter
        assert_eq!(r.calc_x(0).unwrap(), 1);
    }

    #[test]
    fn calc_val() {
        let v = v(X);

        assert_eq!(v.calc_x(33.0).unwrap(), 33.0);
    }

    #[test]
    fn calc_val_wrong_var() {
        let v = v(Y);

        assert_eq!(v.calc_x(33.0f32), Err(VarNotProvided("rad::var::Y")));
    }

    #[test]
    fn calc_add_cos() {
        let cos = cos(v(X)) + cos(v(X));

        assert_eq!(cos.calc_x(0.0).unwrap(), 2.0);
    }

    #[test]
    fn calc_mul_cos() {
        let cos = cos(v(X)) * c(3.0);

        assert_eq!(cos.calc_x(0.0).unwrap(), 3.0);
    }

    #[test]
    fn addition_with_sin() {
        let addition = c(1.0) + sin(v(X));
        assert_eq!(addition.calc_x(0.0).unwrap(), 1.0);

        let r = addition.diff::<X>();

        assert_eq!(r, c(0.0) + mul(cos(v(X)), c(1.0)));
        assert_eq!(r.calc_x(0.0).unwrap(), 1.0);
    }

    #[test]
    fn weird_one() {
        let val = cos(c(1.0f32) + sin(v(X)));

        let r = val.diff::<X>();

        // println!("{r}");
        assert_eq!(
            r,
            -sin(c(1.0) + sin(v(X))) * (c(0.0) + (cos(v(X)) * c(1.0)))
        );
    }

    #[test]
    fn chain_rule() {
        let val = cos(cos(cos(v(X)))) + c(1.0f32);

        let r = val.diff::<X>();

        let e = (-sin(cos(cos(v(X)))) * (-sin(cos(v(X))) * (-sin(v(X)) * c(1.0)))) + c(0.0);

        assert_eq!(r, e);
    }
}

#[cfg(test)]
mod multi_var_tests {
    use super::prelude::*;

    #[test]
    fn simple() {
        let val = cos(v(X)) + sin(v(Y)) + c(1.0f32);

        let rx = val.diff::<X>();

        assert_eq!(rx, ((-sin(v(X)) * c(1.0)) + (cos(v(Y)) * c(0.0))) + c(0.0));

        let ry = val.diff::<Y>();

        assert_eq!(ry, ((-sin(v(X)) * c(0.0)) + (cos(v(Y)) * c(1.0))) + c(0.0));

        assert_eq!(
            val.calc(vals(X, 0.0).add(Y, std::f32::consts::PI / 2.0).build())
                .unwrap(),
            3.0
        );
        // we can also use helper functions
        assert_eq!(rx.calc_xy(0.0, 0.0).unwrap(), 0.0);
        assert_eq!(ry.calc_xy(0.0, 0.0).unwrap(), 1.0);
    }

    #[test]
    fn chain_rule() {
        let val = cos(cos(cos(v(X) * v(Y)))) + c(1.0f32);

        let r = val.diff::<X>();

        let e = (-sin(cos(cos(v(X) * v(Y))))
            * (-sin(cos(v(X) * v(Y))) * (-sin(v(X) * v(Y)) * (c(1.0) * v(Y) + v(X) * c(0.0)))))
            + c(0.0f32);

        // since we don't have simplification, the result is
        //         ((-sin(cos(cos((X * Y)))) * (-sin(cos((X * Y))) * (-sin((X * Y)) * ((1 * Y) + (X * 0))))) + 0)
        // the result wolframalpha says for (d/dx) cos(cos(cos(x*y))) + 1 is
        //         -y sin(x y) sin(cos(x y)) sin(cos(cos(x y)))
        // which we can see is the same

        // and thus we can see that chain rule works correctly :)

        assert_eq!(r, e);
    }
}
