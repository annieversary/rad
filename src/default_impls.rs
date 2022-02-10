use crate::{
    differentiables::{cos::Cosable, sin::Sinable},
    domain::Domain,
};

macro_rules! ints {
    ($( $t:ident ),*) => {
      $(
        impl Domain for $t {
          const ZERO: Self = 0;
          const ONE: Self = 1;
        }
      )*
    };
}
ints!(u8, i8, u16, i16, u32, i32, u64, i64);

macro_rules! floats {
    ($( $t:ident ),*) => {
      $(
        impl Domain for $t {
          const ZERO: Self = 0.0;
          const ONE: Self = 1.0;
        }

        impl Sinable for $t {
          fn sin(v: Self) -> Self {
            v.sin()
          }
        }
        impl Cosable for $t {
          fn cos(v: Self) -> Self {
            v.cos()
          }
        }
      )*
    };
}

floats!(f32, f64);
