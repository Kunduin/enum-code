# enum-code

[![Crates.io](https://img.shields.io/crates/v/enum-code)](https://crates.io/crates/enum-code)
[![docs.rs](https://img.shields.io/docsrs/enum-code/latest)](https://docs.rs/enum-code)
[![GitHub](https://img.shields.io/github/license/kunduin/enum-code)](https://github.com/Kunduin/enum-code)



## Introduction

`enum-code` is a `derive macro` for `enum` types. This library generates code that associates error codes with error types. It can be used in conjunction with the `thiserror` crate. Developers can quickly retrieve error codes by calling the `get_code` method.

## Installation

`enum-code` is published on Cargo and can be installed using:

```
$ cargo add enum-code
```

## Usage

1. Add the `Code` attribute to the `enum` type:

   ```rust
   #[derive(enum_code::Code)]
   enum TestError {
       #[code(1)]
       Tuple(String),
       #[code(2)]
       Struct { message: String },
       #[code(3)]
       Simple,
   }
   ```

2. Code Generation

   For the `TestError` enum above, an associated `impl TestError` struct is generated, which includes a `get_code` method that returns the corresponding error code based on the variant value.

   ```rust
   impl TestError {
       pub const fn get_code(&self) -> u32 {
           match self {
               TestError::Tuple(..) => 1u32,
               TestError::Struct { .. } => 2u32,
               TestError::Simple => 3u32,
           }
       }
   }
   ```

3. Retrieving Error Codes

   Error codes can be retrieved by calling `get_code`:

   ```rust
   let err = TestError::Tuple("error message".to_owned());
   let code = err.get_code();
   println!("error code: {}", code); // should print 「error code: 1」
   ```

## LICENSE

MIT