#[doc(hidden)]
#[macro_export]
macro_rules! expr_or_default_expr {
    (, $default:expr) => {
        $default
    };
    ($expr:expr, $default:expr) => {
        $expr
    };
}
