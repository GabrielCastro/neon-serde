//!
//! Defines macros for easily exporting functions
//! example:
//! ```rust,no_run
//!     #[macro_use]
//!     extern crate neon;
//!     #[macro_use]
//!     extern crate neon_serde;
//!     #[macro_use]
//!     extern crate serde_derive;
//!
//!     #[derive(Deserialize)]
//!     struct User {
//!         name: String,
//!         age: u16,
//!     }
//!
//!     export! {
//!         fn say_hello(name: String) -> String {
//!             format!("Hello, {}!", name)
//!         }
//!
//!         fn greet(user: User) -> String {
//!             format!("{} is {} years old", user.name, user.age)
//!         }
//!
//!         fn fibonacci(n: i32) -> i32 {
//!             match n {
//!                 1 | 2 => 1,
//!                 n => fibonacci(n - 1) + fibonacci(n - 2)
//!             }
//!         }
//!     }
//!```
//!

#[macro_export]
macro_rules! export {
    (
        $( fn $name:ident($( $arg:ident : $atype:ty ),*) -> $ret:ty $code:block )*
    ) => (
        $(
            fn $name($( $arg: $atype ),*) -> $ret $code
        )*

        register_module!(m, {
            $(
                m.export(stringify!($name), |call| {
                    let scope = call.scope;

                    // Can be done away with a fancier macro
                    let mut _arg_index = 0;

                    $(
                        let $arg = call.arguments.require(scope, _arg_index)?;
                        let $arg: $atype = $crate::from_value(scope, $arg)?;
                        _arg_index += 1;
                    )*

                    let result = $name($( $arg ),*);
                    let handle = $crate::to_value(scope, &result)?;
                    Ok(handle)
                })?;
            )*
            Ok(())
        });
    )
}
