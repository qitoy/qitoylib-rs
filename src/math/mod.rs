mod combi;
pub mod prime;
pub mod montgomery;
mod numeric;
mod matrix;
pub mod algebra;

#[doc(inline)]
pub use combi::Combi;

#[doc(inline)]
pub use numeric::{Gcd, Lcm};

#[doc(inline)]
pub use matrix::*;
