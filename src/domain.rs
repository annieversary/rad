/// the type that we differentiate over
/// will usually be a float or integer number
pub trait Domain: Clone {
    const ZERO: Self;
    const ONE: Self;
}
