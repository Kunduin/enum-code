//! ## Introduction
//!
//! `enum-code` is a `derive macro` for `enum` types. This library generates code that associates error codes with error types. It can be used in conjunction with the `thiserror` crate. Developers can quickly retrieve error codes by calling the `get_code` method.
//!
//! ## Usage
//!
//! #### 1. Add the `Code` attribute to the `enum` type:
//!
//! ```
//! #[derive(enum_code::Code)]
//! enum TestError {
//!     #[code(1)]
//!     Tuple(String),
//!     #[code(2)]
//!     Struct { message: String },
//!     #[code(3)]
//!     Simple,
//! }
//! ```
//!
//! #### 2. Code Generation
//!
//! For the `TestError` enum above, an associated `impl TestError` struct is generated, which includes a `get_code` method that returns the corresponding error code based on the variant value.
//!
//! ```
//! impl TestError {
//!     pub const fn get_code(&self) -> u32 {
//!         match self {
//!             TestError::Tuple(..) => 1u32,
//!             TestError::Struct { .. } => 2u32,
//!             TestError::Simple => 3u32,
//!         }
//!     }
//! }
//! ```
//!
//! #### 3. Retrieving Error Codes
//!
//! Error codes can be retrieved by calling `get_code`:
//!
//! ```
//! let err = TestError::Tuple("error message".to_owned());
//! let code = err.get_code();
//! println!("error code: {}", code); // should print 「error code: 1」
//! ```

use crate::code::parse_code_stream;

mod code;

#[proc_macro_derive(Code, attributes(code))]
pub fn code(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_code_stream(input)
}
