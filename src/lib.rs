#![allow(unused_variables)]
#![deny(unused_mut)]
extern crate serde;
#[macro_use]
extern crate error_chain;
extern crate neon;

pub mod ser;

pub mod errors {
    use serde::{ser, de};
    use std::fmt::Display;
    use std::convert::From;
    use neon;

    error_chain!{
        errors {
            StringTooLong(len: usize) {
                description("String too long for nodejs")
                display("String too long for nodejs len: {}", len)
            }
        }
        foreign_links {
            Js(neon::vm::Throw);
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

    impl From<Error> for neon::vm::Throw {
        fn from(err: Error) -> Self {
            eprintln!("{:?}", err);
            ::neon::vm::Throw
        }
    }

}

pub use ser::to_value;
