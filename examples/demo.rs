#![allow(missing_docs, unused_crate_dependencies)]

use unwind_context::unwind_context;

#[derive(Clone, Debug)]
struct Wrapper<T>(T);

fn main() {
    unwind_context::enable_colors_if_supported();
    app_logic(Wrapper("abc\nbcd".to_owned()), &[1, 2], "secret", false);
}

fn app_logic(value: Wrapper<String>, arr: &[u8], secret: &str, flag: bool) {
    let _ctx = unwind_context!(fn(value.clone(), arr, ..., flag));
    // ...
    let _ = collect_rotations("áöù");
    // ...
    let _ = (value, arr, secret, flag);
}

fn collect_rotations(value: &str) -> Vec<String> {
    let _ctx = unwind_context!(fn(value));
    (0..value.len())
        .map(|mid| rotate_left(value, mid))
        .collect()
}

fn rotate_left(value: &str, mid: usize) -> String {
    let _ctx = unwind_context!(fn(value, mid));
    let (left, right) = split(value, mid);
    format!("{right}{left}")
}

fn split(value: &str, at: usize) -> (&str, &str) {
    let _ctx = unwind_context!(fn(value, at));
    (&value[0..at], &value[at..])
}
