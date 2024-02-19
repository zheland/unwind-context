use core::fmt::{Debug, Formatter, Result as FmtResult, Write as FmtWrite};

use crate::{AnsiColorScheme, DebugAnsiColored};

/// A structure representing an argument name and its value.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnwindContextArg<T> {
    /// Optional argument name.
    pub name: Option<&'static str>,
    /// Argument value.
    pub value: T,
}

impl<T> UnwindContextArg<T> {
    /// Create a new `UnwindContextArg` with the provided name and value.
    #[inline]
    pub fn new(name: Option<&'static str>, value: T) -> Self {
        Self { name, value }
    }
}

impl<T> Debug for UnwindContextArg<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = &self.name {
            write!(f, "{name}: ")?;
        }
        write!(f, "{:?}", self.value)?;
        Ok(())
    }
}

impl<T> DebugAnsiColored for UnwindContextArg<T>
where
    T: Debug,
{
    #[inline]
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult {
        if let Some(name) = &self.name {
            write!(f, "{name}: ")?;
        }
        let mut writer = ColoredWriter {
            writer: f,
            mode: ColoredWriterMode::Default,
            color_scheme,
        };
        write!(writer, "{:?}", self.value)?;
        writer.reset()?;
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ColoredWriter<W> {
    writer: W,
    mode: ColoredWriterMode,
    color_scheme: &'static AnsiColorScheme,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ColoredWriterMode {
    Default,
    Ident,
    Item,
    Boolean,
    Number,
    DoubleQuoted,
    DoubleQuotedEscapeChar,
    DoubleQuotedEscaped,
    SingleQuoted,
    SingleQuotedEscapeChar,
    SingleQuotedEscaped,
    QuotedEnd,
    Brace,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ColoredWriterModeStyle {
    Default,
    Ident,
    Item,
    Boolean,
    Number,
    Quoted,
    Escaped,
    Brace,
}

impl ColoredWriterModeStyle {
    fn ansi_style(&self, color_scheme: &AnsiColorScheme) -> &'static str {
        match self {
            Self::Default => color_scheme.default,
            Self::Ident => color_scheme.ident,
            Self::Item => color_scheme.item,
            Self::Boolean => color_scheme.boolean,
            Self::Number => color_scheme.number,
            Self::Quoted => color_scheme.quoted,
            Self::Escaped => color_scheme.escaped,
            Self::Brace => color_scheme.value_braces,
        }
    }
}

impl<W> ColoredWriter<W>
where
    W: FmtWrite,
{
    fn reset(&mut self) -> FmtResult {
        if self.mode.style() != ColoredWriterModeStyle::Default {
            self.writer.write_str(self.color_scheme.default)?;
            self.mode = ColoredWriterMode::Default;
        }
        Ok(())
    }
}

impl ColoredWriterMode {
    fn style(self) -> ColoredWriterModeStyle {
        match self {
            Self::Default => ColoredWriterModeStyle::Default,
            Self::Ident => ColoredWriterModeStyle::Ident,
            Self::Item => ColoredWriterModeStyle::Item,
            Self::Boolean => ColoredWriterModeStyle::Boolean,
            Self::Number => ColoredWriterModeStyle::Number,
            Self::DoubleQuoted | Self::SingleQuoted | Self::QuotedEnd => {
                ColoredWriterModeStyle::Quoted
            }
            Self::DoubleQuotedEscapeChar
            | Self::DoubleQuotedEscaped
            | Self::SingleQuotedEscapeChar
            | Self::SingleQuotedEscaped => ColoredWriterModeStyle::Escaped,
            Self::Brace => ColoredWriterModeStyle::Brace,
        }
    }
}

impl<W> FmtWrite for ColoredWriter<W>
where
    W: FmtWrite,
{
    // Not the perfect, but a simple and quite performant implementation
    // that provides sufficient coloring.
    #[allow(clippy::too_many_lines)]
    fn write_str(&mut self, s: &str) -> FmtResult {
        for (offset, ch) in s.char_indices() {
            let prev_style = self.mode.style();
            self.mode = match self.mode {
                ColoredWriterMode::Default
                | ColoredWriterMode::QuotedEnd
                | ColoredWriterMode::Brace => match ch {
                    '0'..='9' | '+' | '-' | '.' => ColoredWriterMode::Number,
                    '(' | ')' | '[' | ']' | '{' | '}' => ColoredWriterMode::Brace,
                    '_' => ColoredWriterMode::Ident,
                    '"' => ColoredWriterMode::DoubleQuoted,
                    '\'' => ColoredWriterMode::SingleQuoted,
                    'A'..='Z' => ColoredWriterMode::Item,
                    _ => {
                        if ch.is_alphanumeric() {
                            // Look ahead and check for `true` and `false` keywords.
                            if match_true_ident(s, offset) || match_false_ident(s, offset) {
                                ColoredWriterMode::Boolean
                            } else {
                                ColoredWriterMode::Ident
                            }
                        } else {
                            ColoredWriterMode::Default
                        }
                    }
                },
                ColoredWriterMode::Ident | ColoredWriterMode::Item => match ch {
                    '(' | ')' | '[' | ']' | '{' | '}' => ColoredWriterMode::Brace,
                    '#' | '_' => self.mode,
                    '"' => ColoredWriterMode::DoubleQuoted,
                    '\'' => ColoredWriterMode::SingleQuoted,
                    ch => {
                        if ch.is_alphanumeric() {
                            self.mode
                        } else {
                            ColoredWriterMode::Default
                        }
                    }
                },
                ColoredWriterMode::Boolean => match ch {
                    '0'..='9' | '+' | '-' | '.' => ColoredWriterMode::Number,
                    '(' | ')' | '[' | ']' | '{' | '}' => ColoredWriterMode::Brace,
                    '#' | '_' => ColoredWriterMode::Ident,
                    '"' => ColoredWriterMode::DoubleQuoted,
                    '\'' => ColoredWriterMode::SingleQuoted,
                    ch => {
                        if ch.is_alphanumeric() {
                            ColoredWriterMode::Boolean
                        } else {
                            ColoredWriterMode::Default
                        }
                    }
                },
                ColoredWriterMode::Number => match ch {
                    '0'..='9' | '+' | '-' | '.' | '_' => ColoredWriterMode::Number,
                    '(' | ')' | '[' | ']' | '{' | '}' => ColoredWriterMode::Brace,
                    '"' => ColoredWriterMode::DoubleQuoted,
                    '\'' => ColoredWriterMode::SingleQuoted,
                    ch => {
                        if ch.is_alphanumeric() {
                            ColoredWriterMode::Ident
                        } else {
                            ColoredWriterMode::Default
                        }
                    }
                },
                ColoredWriterMode::DoubleQuoted | ColoredWriterMode::DoubleQuotedEscaped => {
                    match ch {
                        '"' => ColoredWriterMode::QuotedEnd,
                        '\\' => ColoredWriterMode::DoubleQuotedEscapeChar,
                        _ => ColoredWriterMode::DoubleQuoted,
                    }
                }
                ColoredWriterMode::DoubleQuotedEscapeChar => ColoredWriterMode::DoubleQuotedEscaped,
                ColoredWriterMode::SingleQuoted | ColoredWriterMode::SingleQuotedEscaped => {
                    match ch {
                        '\'' => ColoredWriterMode::QuotedEnd,
                        '\\' => ColoredWriterMode::SingleQuotedEscapeChar,
                        _ => ColoredWriterMode::SingleQuoted,
                    }
                }
                ColoredWriterMode::SingleQuotedEscapeChar => ColoredWriterMode::SingleQuotedEscaped,
            };
            let style = self.mode.style();
            if prev_style != style {
                self.writer.write_str(style.ansi_style(self.color_scheme))?;
            }
            self.writer.write_char(ch)?;
        }
        Ok(())
    }
}

fn match_true_ident(s: &str, offset: usize) -> bool {
    s.as_bytes().get(offset..offset.saturating_add(4)) == Some(b"true")
        && s.as_bytes()
            .get(offset.saturating_add(4))
            .map_or(true, |&ch| !ch.is_ascii_alphanumeric() && ch != b'_')
}

fn match_false_ident(s: &str, offset: usize) -> bool {
    s.as_bytes().get(offset..offset.saturating_add(5)) == Some(b"false")
        && s.as_bytes()
            .get(offset.saturating_add(5))
            .map_or(true, |&ch| !ch.is_ascii_alphanumeric() && ch != b'_')
}

#[cfg(test)]
mod tests {
    use core::fmt::{Debug, Error as FmtError};
    use core::marker::PhantomData;

    use crate::arg::{match_false_ident, match_true_ident};
    use crate::test_common::{arg, colored_arg, TEST_ANSI_COLOR_SCHEME};
    use crate::test_util::{debug_fmt, TransparentDebug};
    use crate::{AnsiColored, UnwindContextArg};

    #[derive(Clone, Debug)]
    struct Wrapper<T> {
        _first: T,
        _second: T,
        _phantom: PhantomData<u32>,
    }

    fn fmt_str_as_arg<'a>(buffer: &'a mut [u8], value: &'static str) -> Result<&'a str, FmtError> {
        debug_fmt(
            buffer,
            &AnsiColored::new(
                UnwindContextArg::new(None, TransparentDebug(value)),
                &TEST_ANSI_COLOR_SCHEME,
            ),
        )
    }

    #[test]
    fn test_match_true_ident() {
        assert!(!match_true_ident("", 0));
        assert!(!match_true_ident("a", 0));
        assert!(!match_true_ident("false", 0));
        assert!(!match_true_ident("abcd", 0));
        assert!(match_true_ident("true", 0));
        assert!(match_true_ident("true.false", 0));
        assert!(match_true_ident("true!false", 0));
        assert!(match_true_ident("true-false", 0));
        assert!(match_true_ident("true false", 0));
        assert!(!match_true_ident("truetrue", 0));
        assert!(!match_true_ident("truest", 0));
        assert!(!match_true_ident("true1", 0));
        assert!(!match_true_ident("true_", 0));
        assert!(match_true_ident("(true)", 1));
        assert!(match_true_ident("((true))", 2));
    }

    #[test]
    fn test_match_false_ident() {
        assert!(!match_false_ident("", 0));
        assert!(!match_false_ident("a", 0));
        assert!(!match_false_ident("true", 0));
        assert!(!match_false_ident("abcde", 0));
        assert!(match_false_ident("false", 0));
        assert!(match_false_ident("false.true", 0));
        assert!(match_false_ident("false!true", 0));
        assert!(match_false_ident("false-true", 0));
        assert!(match_false_ident("false true", 0));
        assert!(!match_false_ident("falsefalse", 0));
        assert!(!match_false_ident("falsest", 0));
        assert!(!match_false_ident("false1", 0));
        assert!(!match_false_ident("false_", 0));
        assert!(match_false_ident("(false)", 1));
        assert!(match_false_ident("((false))", 2));
    }

    #[test]
    fn test_arg_fmt() {
        let mut buffer = [0; 128];
        assert_eq!(debug_fmt(&mut buffer, &arg(None, "value")), Ok("\"value\""));
        assert_eq!(
            debug_fmt(&mut buffer, &arg(Some("foo"), 123)),
            Ok("foo: 123")
        );
        assert_eq!(
            debug_fmt(&mut buffer, &arg(Some("foo"), "bar\n-\"-'-\"bar")),
            Ok("foo: \"bar\\n-\\\"-'-\\\"bar\"")
        );
        assert_eq!(
            debug_fmt(&mut buffer, &arg(Some("foo"), 'a')),
            Ok("foo: 'a'")
        );
        assert_eq!(
            debug_fmt(
                &mut buffer,
                &arg(
                    Some("foo"),
                    Wrapper {
                        _first: true,
                        _second: false,
                        _phantom: PhantomData,
                    }
                )
            ),
            Ok("foo: Wrapper { _first: true, _second: false, _phantom: PhantomData<u32> }")
        );
    }

    #[test]
    fn test_arg_colored_fmt() {
        let mut buffer = [0; 256];
        assert_eq!(
            debug_fmt(&mut buffer, &colored_arg(None, "value")),
            Ok("{QUOT}\"value\"{DEF}")
        );
        assert_eq!(
            debug_fmt(&mut buffer, &colored_arg(Some("foo"), 123)),
            Ok("foo: {NUM}123{DEF}")
        );
        assert_eq!(
            debug_fmt(&mut buffer, &colored_arg(Some("foo"), "bar\n-\"-'-\"bar")),
            Ok(concat!(
                "foo: ",
                "{QUOT}\"bar",
                "{ESC}\\n",
                "{QUOT}-",
                "{ESC}\\\"",
                "{QUOT}-'-",
                "{ESC}\\\"",
                "{QUOT}bar\"",
                "{DEF}"
            ))
        );
        assert_eq!(
            debug_fmt(&mut buffer, &colored_arg(Some("foo"), 'a')),
            Ok("foo: {QUOT}'a'{DEF}")
        );
        assert_eq!(
            debug_fmt(
                &mut buffer,
                &colored_arg(
                    Some("foo"),
                    Wrapper {
                        _first: true,
                        _second: false,
                        _phantom: PhantomData,
                    }
                )
            ),
            Ok(concat!(
                "foo: ",
                "{ITEM}Wrapper",
                "{DEF} {BRACE}{",
                "{DEF} ",
                "{IDENT}_first{DEF}: {BOOL}true{DEF}, ",
                "{IDENT}_second{DEF}: {BOOL}false{DEF}, ",
                "{IDENT}_phantom{DEF}: ",
                "{ITEM}PhantomData{DEF}<{IDENT}u32{DEF}> ",
                "{BRACE}}",
                "{DEF}"
            ))
        );
    }

    #[test]
    fn test_complex_colored_fmt() {
        use fmt_str_as_arg as f;

        let mut buffer = [0; 64];
        let buf = &mut buffer;

        assert_eq!(f(buf, "123"), Ok("{NUM}123{DEF}"));
        assert_eq!(f(buf, "()"), Ok("{BRACE}(){DEF}"));
        assert_eq!(f(buf, "_foo"), Ok("{IDENT}_foo{DEF}"));
        assert_eq!(f(buf, "\"foo\""), Ok("{QUOT}\"foo\"{DEF}"));
        assert_eq!(f(buf, "'foo'"), Ok("{QUOT}'foo'{DEF}"));
        assert_eq!(f(buf, "Bar"), Ok("{ITEM}Bar{DEF}"));
        assert_eq!(f(buf, "BAR"), Ok("{ITEM}BAR{DEF}"));
        assert_eq!(f(buf, "true"), Ok("{BOOL}true{DEF}"));
        assert_eq!(f(buf, "false"), Ok("{BOOL}false{DEF}"));
        assert_eq!(f(buf, "foo"), Ok("{IDENT}foo{DEF}"));
        assert_eq!(f(buf, "&"), Ok("&"));

        assert_eq!(f(buf, "foo()"), Ok("{IDENT}foo{BRACE}(){DEF}"));
        assert_eq!(f(buf, "foo_bar"), Ok("{IDENT}foo_bar{DEF}"));
        assert_eq!(f(buf, "r#raw"), Ok("{IDENT}r#raw{DEF}"));
        assert_eq!(f(buf, "b'1'"), Ok("{IDENT}b{QUOT}'1'{DEF}"));
        assert_eq!(f(buf, "b\"1\""), Ok("{IDENT}b{QUOT}\"1\"{DEF}"));
        assert_eq!(f(buf, "foo123"), Ok("{IDENT}foo123{DEF}"));
        assert_eq!(f(buf, "foo&"), Ok("{IDENT}foo{DEF}&"));

        assert_eq!(f(buf, "true+"), Ok("{BOOL}true{NUM}+{DEF}"));
        assert_eq!(f(buf, "[true]"), Ok("{BRACE}[{BOOL}true{BRACE}]{DEF}"));
        assert_eq!(f(buf, "true#"), Ok("{BOOL}true{IDENT}#{DEF}"));
        assert_eq!(f(buf, "false\"\""), Ok("{BOOL}false{QUOT}\"\"{DEF}"));
        assert_eq!(f(buf, "false\'\'"), Ok("{BOOL}false{QUOT}''{DEF}"));
        assert_eq!(f(buf, "false&"), Ok("{BOOL}false{DEF}&"));

        assert_eq!(f(buf, "123"), Ok("{NUM}123{DEF}"));
        assert_eq!(f(buf, "2[]"), Ok("{NUM}2{BRACE}[]{DEF}"));
        assert_eq!(f(buf, "3\"\""), Ok("{NUM}3{QUOT}\"\"{DEF}"));
        assert_eq!(f(buf, "4\'\'"), Ok("{NUM}4{QUOT}''{DEF}"));
        assert_eq!(f(buf, "5a"), Ok("{NUM}5{IDENT}a{DEF}"));
        assert_eq!(f(buf, "6^7"), Ok("{NUM}6{DEF}^{NUM}7{DEF}"));

        assert_eq!(f(buf, "\"\\\"\""), Ok("{QUOT}\"{ESC}\\\"{QUOT}\"{DEF}"));
        assert_eq!(f(buf, "'\\''"), Ok("{QUOT}'{ESC}\\'{QUOT}'{DEF}"));
        assert_eq!(f(buf, ""), Ok(""));
    }

    #[test]
    fn test_arg_failed_fmt() {
        let arg = arg(Some("foo"), TransparentDebug("[1, 2, 3]"));

        let mut buffer = [0; 64];
        let len = debug_fmt(&mut buffer, &arg).unwrap().len();
        for len in 0..len {
            assert_eq!(debug_fmt(&mut buffer[0..len], &arg), Err(FmtError));
        }
    }

    #[test]
    fn test_arg_failed_colored_fmt() {
        let arg = colored_arg(Some("foo"), TransparentDebug("[1, 2, 3]"));

        let mut buffer = [0; 64];
        let len = debug_fmt(&mut buffer, &arg).unwrap().len();
        for len in 0..len {
            assert_eq!(debug_fmt(&mut buffer[0..len], &arg), Err(FmtError));
        }
    }
}
