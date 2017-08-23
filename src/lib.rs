#![allow(unused_variables)]
#![deny(unused_mut)]
extern crate cast;
#[macro_use]
extern crate error_chain;
extern crate neon;
extern crate serde;

pub mod ser;
pub mod de;

pub mod errors {
    use serde::{de, ser};
    use std::fmt::Display;
    use std::convert::From;
    use neon;

    error_chain!{
        errors {
            StringTooLong(len: usize) {
                description("String too long for nodejs")
                display("String too long for nodejs len: {}", len)
            }
            UnableToCoerce(to_type: &'static str) {
                description("Unable to coerce")
                display("Unable to coerce value to type: {}", to_type)
            }
            EmptyString {
                description("EmptyString")
                display("EmptyString")
            }
            StringTooLongForChar(len: usize) {
                description("String too long to be a char")
                display("String too long to be a char expected len: 1 got len: {}", len)
            }
            ExpectingNull
            InvalidKeyType(key: String) {
                description("InvalidKeyType")
                display("key: '{}'", key)
            }
            ArrayIndexOutOfBounds(index: u32, length: u32) {
                description("ArrayIndexOutOfBounds")
                display(
                    "ArrayIndexOutOfBounds: attempt to access ({}) size: ({})",
                    index,
                    length
                )
            }
            NotImplemented(name: &'static str) {
                description("Not Implemented")
                display("Not Implemented: '{}'", name)
            }
        }
        foreign_links {
            Js(neon::vm::Throw);
            NumberCastError(::cast::Error);
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
pub use de::from_handle;
