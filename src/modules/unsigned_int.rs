use serde::{de::DeserializeOwned, Serialize};

// Sealed trait (same as before)
mod sealed {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

pub trait UnsignedInt:
    sealed::Sealed
    + Copy
    + Clone
    + std::fmt::Debug
    + Serialize
    + DeserializeOwned
    + std::hash::Hash
    + Eq // + other ops if needed (e.g., Add, From<u8>)
{
    fn to_usize(&self) -> usize; // NEW: Infallible conversion to usize

    // Optional: Symmetric from usize (infallible, assuming values fit)
    fn from_usize(value: usize) -> Self;
}

impl UnsignedInt for usize {
    fn to_usize(&self) -> usize {
        *self
    }

    fn from_usize(value: usize) -> Self {
        value
    }
}
// Impls for smaller types (always safe)
impl UnsignedInt for u8 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u8 // Truncates if too large; add checks if needed
    }
}

impl UnsignedInt for u16 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u16
    }
}

impl UnsignedInt for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u32
    }
}

// Conditional impl for u64: Only on 64-bit systems
#[cfg(target_pointer_width = "64")]
impl UnsignedInt for u64 {
    fn to_usize(&self) -> usize {
        *self as usize // Safe: same size
    }

    fn from_usize(value: usize) -> Self {
        value as u64
    }
}
