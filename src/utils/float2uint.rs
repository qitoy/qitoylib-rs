pub trait ToUint {
    type Uint;
    fn to_uint(self) -> Self::Uint;
}

pub trait ToFloat {
    type Float;
    fn to_float(self) -> Self::Float;
}

macro_rules! impl_f2u {
    ($( $u:ty, $f:ty, $b:expr ),*) => {
        $(
            impl ToUint for $f {
                type Uint = $u;
                fn to_uint(self) -> Self::Uint {
                    let u = self.to_bits();
                    if u >> $b == 1 { !u } else { u ^ 1 << $b }
                }
            }
            impl ToFloat for $u {
                type Float = $f;
                fn to_float(self) -> Self::Float {
                    <$f>::from_bits(if self >> $b == 1 { self ^ 1 << $b } else { !self })
                }
            }
         )*
    }
}

impl_f2u!(u32, f32, 31, u64, f64, 63);
