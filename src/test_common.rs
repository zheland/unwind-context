#[cfg(feature = "std")]
use std::sync::Mutex;

use crate::test_util::PatternMatcher;
use crate::{AnsiColorScheme, AnsiColored, UnwindContextArg, UnwindContextArgs};

#[cfg(feature = "std")]
// Modifying and checking the values of global and environment variables
// requires a global lock to prohibit parallel execution in different tests.
pub static SERIAL_TEST: Mutex<()> = Mutex::new(());

pub static TEST_ANSI_COLOR_SCHEME: AnsiColorScheme = AnsiColorScheme {
    default: "{DEF}",
    location: "{LOC}",
    fn_keyword: "{FN}",
    func_name: "{FN_NAME}",
    func_braces: "{FN_BRACE}",
    value_braces: "{BRACE}",
    ident: "{IDENT}",
    item: "{ITEM}",
    boolean: "{BOOL}",
    number: "{NUM}",
    quoted: "{QUOT}",
    escaped: "{ESC}",
};

pub fn arg<T>(name: Option<&'static str>, value: T) -> UnwindContextArg<T> {
    UnwindContextArg::new(name, value)
}

pub fn colored_arg<T>(name: Option<&'static str>, value: T) -> AnsiColored<UnwindContextArg<T>> {
    AnsiColored::new(UnwindContextArg::new(name, value), &TEST_ANSI_COLOR_SCHEME)
}

pub fn args<T>(args: T) -> UnwindContextArgs<T> {
    UnwindContextArgs::new(args)
}

pub fn colored_args<T>(args: T) -> AnsiColored<UnwindContextArgs<T>> {
    AnsiColored::new(UnwindContextArgs::new(args), &TEST_ANSI_COLOR_SCHEME)
}

#[track_caller]
pub fn check_location_part(
    output: &mut &str,
    location_prefix: &str,
    location_suffix: &str,
    expected_file: &str,
    min_line: u32,
    max_line: u32,
) {
    #[cfg(feature = "std")]
    let _ctx = crate::unwind_context!(
        fn(*output, location_prefix, location_suffix, expected_file, min_line, max_line)
    );

    output.expect_str("    at ").unwrap();
    output.expect_str(location_prefix).unwrap();
    let file = output.read_until(":").unwrap();
    assert_eq!(file, expected_file);
    let line: u32 = output.read_until(":").unwrap().parse().unwrap();
    assert!(line > min_line);
    assert!(line < max_line);
    if location_suffix.is_empty() {
        let _column = output.read_until("\n").unwrap();
    } else {
        let _column = output.read_until(location_suffix).unwrap();
        output.expect_str("\n").unwrap();
    }
}
