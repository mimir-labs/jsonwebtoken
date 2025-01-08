mod core {
    #[cfg(not(feature = "std"))]
    pub use core::*;
    #[cfg(feature = "std")]
    pub use std::*;
}

pub use self::core::fmt;
pub use self::core::result;
pub use self::core::str;

#[cfg(not(feature = "std"))]
pub use alloc::format;
#[cfg(feature = "std")]
pub use std::format;
pub mod string {
    #[cfg(not(feature = "std"))]
    pub use alloc::string::{FromUtf8Error, String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::{FromUtf8Error, String, ToString};
}
pub mod vec {
    #[cfg(not(feature = "std"))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

pub mod sync {
    #[cfg(not(feature = "std"))]
    pub use alloc::sync::Arc;
    #[cfg(feature = "std")]
    pub use std::sync::Arc;
}

pub mod boxed {
    #[cfg(not(feature = "std"))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;
}
