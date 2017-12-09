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
