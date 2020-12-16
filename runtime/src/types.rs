/// Represent type in marrow.
pub enum Type<'a> {
    /// uint8_t
    U8(u8),
    /// uint16_t
    U16(u16),
    /// uint32_t
    U32(u32),
    /// uint64_t
    U64(u64),
    /// int8_t
    I8(i8),
    /// int16_t
    I16(i16),
    /// int32_t
    I32(i32),
    /// int64_t
    I64(i64),
    /// size_t
    MemSize(usize),
    /// offset_t
    MemOffset(isize),
    /// (uint8_t *, size_t)
    Bytes(&'a [u8]),
    /// uint8_t *, bytes end with `\0`.
    Str(&'a str),
}

macro_rules! define_froms {
    ($rt:tt, $et:ident) => {
        impl<'a> From<$rt> for Type<'a> {
            fn from(v: $rt) -> Self {
                Type::$et(v)
            }
        }
    };
}

define_froms!(u8, U8);
define_froms!(u16, U16);
define_froms!(u32, U32);
define_froms!(u64, U64);
define_froms!(i8, I8);
define_froms!(i16, I16);
define_froms!(i32, I32);
define_froms!(i64, I64);
define_froms!(usize, MemSize);
define_froms!(isize, MemOffset);

impl<'a> From<&'a [u8]> for Type<'a> {
    fn from(v: &'a [u8]) -> Self {
        Type::Bytes(v)
    }
}

impl<'a> From<&'a str> for Type<'a> {
    fn from(v: &'a str) -> Self {
        Type::Str(v)
    }
}
