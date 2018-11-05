#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

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

    /// calculate fibonacci recursively
    fn fibonacci(n: i32) -> i32 {
        match n {
            1 | 2 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}
