#![cfg_attr(not(feature = "std"), no_std)]

//! Create and parses JWT (JSON Web Tokens)
//!
//! Documentation:  [stable](https://docs.rs/jsonwebtoken/)
#![deny(missing_docs)]

extern crate alloc;

mod std_lib;

mod algorithms;
/// Lower level functions, if you want to do something other than JWTs
#[cfg(feature = "crypto")]
pub mod crypto;
#[cfg(feature = "crypto")]
mod decoding;
#[cfg(feature = "crypto")]
mod encoding;
/// All the errors that can be encountered while encoding/decoding JWTs
pub mod errors;
mod header;
pub mod jwk;
#[cfg(feature = "use_pem")]
mod pem;
#[cfg(feature = "crypto")]
mod serialization;
#[cfg(feature = "validation")]
mod validation;

pub use algorithms::Algorithm;
#[cfg(feature = "crypto")]
pub use decoding::{decode, decode_header, DecodingKey, TokenData};
#[cfg(feature = "crypto")]
pub use encoding::{encode, EncodingKey};
pub use header::Header;
#[cfg(feature = "validation")]
pub use validation::{get_current_timestamp, Validation};
