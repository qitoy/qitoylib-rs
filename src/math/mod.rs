mod combi;
pub mod prime;
pub mod montgomery;
mod numeric;

#[doc(inline)]
pub use combi::Combi;

#[doc(inline)]
pub use numeric::{Gcd, Lcm};
