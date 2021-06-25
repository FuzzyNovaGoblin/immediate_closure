#[macro_export]
macro_rules! CR {
    {$( $x:expr) *} => {
        (||
            $($x)*
            )()
    };
}
