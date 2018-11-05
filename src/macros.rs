//!
//! Defines macros for easily exporting functions
//!

#[macro_export]
macro_rules! export {

    ($(
        $(#[$func_meta:meta])*
        fn $name:ident($( $arg:ident : $atype:ty ),*) -> $ret:ty $code:block
    )*) => (
        $(
            #[allow(non_snake_case)]
            $(#[$func_meta])*
            fn $name($( $arg: $atype ),*) -> $ret $code
        )*

        register_module!(mut m, {
            $(
                m.export_function(stringify!($name), |mut cx| {
                    // Can be done away with a fancier macro
                    let mut _arg_index = 0;

                    $(
                        let $arg = cx.argument_opt(_arg_index);
                        let $arg: $atype = $crate::from_value_opt(&mut cx, $arg)?;
                        _arg_index += 1;
                    )*

                    let result = $name($( $arg ),*);
                    let handle = $crate::to_value(&mut cx, &result)?;
                    Ok(handle)
                })?;
            )*
            Ok(())
        });
    )
}
