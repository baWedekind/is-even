use is_odd::IsOdd;

pub trait IsEven {
    fn is_even(&self) -> bool;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl IsEven for $t {
                fn is_even(&self) -> bool {
                    !self.is_odd()
                }
            }
        )*
    };
}

prim_impl!(i8 u8 i16 u16 i32 u32 i64 u64);