#[doc(hidden)]
/// Strips module path from a function name.
///
/// This is an auxiliary function and is used in [`func_name!`] macro.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     unwind_context::func_name_from_item_type_name(
///         "unwind_context",
///         "unwind_context::func1::Item",
///     ),
///     "func1"
/// );
/// ```
///
/// [`func_name!`]: macro@crate::func_name
#[must_use]
pub fn func_name_from_item_type_name(
    module_path: &'static str,
    subitem: &'static str,
) -> &'static str {
    let name = str::strip_suffix(subitem, "::Item").unwrap_or(subitem);
    let name = str::strip_suffix(name, "::{{closure}}").unwrap_or(name);
    let name = str::strip_prefix(name, module_path).unwrap_or(name);
    let name = str::strip_prefix(name, "::").unwrap_or(name);
    name
}

/// Returns the name of the function where the macro is invoked. Returns a
/// `&'static str`.
///
/// # Note
///
/// This is intended for diagnostic use and the exact output is not guaranteed.
/// It provides a best-effort description, but the output may change between
/// versions of the compiler.
///
/// In short: use this for debugging, avoid using the output to affect program
/// behavior.
///
/// # Examples
///
/// ```
/// let current_function_name = unwind_context::func_name!();
/// println!("defined in function: {current_function_name}");
/// ```
#[macro_export]
macro_rules! func_name {
    () => {{
        struct Item;
        let module_path = ::core::module_path!();
        let item_type_name = ::core::any::type_name::<Item>();

        $crate::func_name_from_item_type_name(module_path, item_type_name)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_func_name() {
        fn foo() -> &'static str {
            func_name!()
        }
        fn bar() -> &'static str {
            func_name!()
        }
        fn baz() -> &'static str {
            func_name!()
        }

        assert!(foo().contains("foo"));
        assert!(bar().contains("bar"));
        assert!(baz().contains("baz"));
    }
}
