#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u16,
}

export! {

    /// Say hello based on a persons name
    fn say_hello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    /// Say how old someone is
    fn greet(user: User) -> String {
        format!("{} is {} years old", user.name, user.age)
    }

    /// Say how old someone is, if they exist
    fn maybe_say_hello(user: Option<User>) -> Option<String> {
        user.map(greet)
    }

    /// Sorts the bytes in a string
    /// use `serde_bytes::ByteBuf` to return a `Buffer` in node
    /// a `Vec<u8>` will be an array
    fn sort_utf8_bytes(str: String) -> serde_bytes::ByteBuf {
        let mut bytes = str.into_bytes();
        bytes.sort();
        serde_bytes::ByteBuf::from(bytes)
    }

    /// using `serde_bytes::ByteBuf` will make passing an array
    /// of numbers an error
    ///
    /// note: `-> ()` is NOT optional
    fn expect_buffer_only(_buff: serde_bytes::ByteBuf) -> () {
        // code
    }

    /// using `Vec<u8>` not accept a buffer
    fn expect_array(_buff: Vec<u8>) -> () {
        // code
    }

    /// calculate fibonacci recursively
    fn fibonacci(n: i32) -> i32 {
        match n {
            1 | 2 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}
