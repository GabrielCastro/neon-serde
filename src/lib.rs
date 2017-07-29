#![allow(unused_variables)]
extern crate serde;
#[macro_use]
extern crate error_chain;
extern crate neon;

pub mod ser;

pub mod errors {
    use serde::{ser, de};
    use std::fmt::Display;

    error_chain!{
        errors {
            StringTooLong(len: usize) {
                description("String too long for nodejs")
                display("String too long for nodejs len: {}", len)
            }
        }
    }

    impl ser::Error for Error {
        fn custom<T: Display>(msg: T) -> Self {
            ErrorKind::Msg(msg.to_string()).into()
        }
    }

    impl de::Error for Error {
        fn custom<T: Display>(msg: T) -> Self {
            ErrorKind::Msg(msg.to_string()).into()
        }
    }

}

pub use ser::to_value;
