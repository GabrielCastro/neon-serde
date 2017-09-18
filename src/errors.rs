use error_chain::ChainedError;
use neon;
use serde::{de, ser};
use std::convert::From;
use std::fmt::Display;

error_chain! {
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
        if let ErrorKind::Js(_) = *err.kind() {
            return neon::vm::Throw
        };
        let msg = format!("{}", err.display_chain());
        neon::js::error::JsError::throw::<()>(neon::js::error::Kind::Error, &msg).unwrap_err()
    }
}
