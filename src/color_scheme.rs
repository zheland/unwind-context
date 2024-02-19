/// The default ANSI color scheme, which is used if colorization is enabled but
/// no custom color scheme is set.
pub static DEFAULT_ANSI_COLOR_SCHEME: AnsiColorScheme = AnsiColorScheme {
    default: "\u{1b}[0m",
    location: "\u{1b}[94m",
    fn_keyword: "\u{1b}[33m",
    func_name: "\u{1b}[93m",
    func_braces: "\u{1b}[0m",
    value_braces: "\u{1b}[0m",
    ident: "\u{1b}[0;33m",
    item: "\u{1b}[0;33m",
    boolean: "\u{1b}[1;93m",
    number: "\u{1b}[0;96m",
    quoted: "\u{1b}[0;32m",
    escaped: "\u{1b}[0;95m",
};

/// A structure representing an ANSI color scheme used by [`DebugAnsiColored`]
/// formatter.
///
/// [`DebugAnsiColored`]: crate::DebugAnsiColored
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AnsiColorScheme {
    /// The ANSI escape sequence used for default text styling.
    pub default: &'static str,
    /// The ANSI escape sequence used before code location.
    pub location: &'static str,
    /// The ANSI escape sequence used before `fn` keyword.
    pub fn_keyword: &'static str,
    /// The ANSI escape sequence used before function name.
    pub func_name: &'static str,
    /// The ANSI escape sequence used before function braces.
    pub func_braces: &'static str,
    /// The ANSI escape sequence used before any value braces.
    pub value_braces: &'static str,
    /// The ANSI escape sequence used before identifiers.
    pub ident: &'static str,
    /// The ANSI escape sequence used before struct, enum and const names.
    pub item: &'static str,
    /// The ANSI escape sequence used before `false` or `true` keywords.
    pub boolean: &'static str,
    /// The ANSI escape sequence used before numbers.
    pub number: &'static str,
    /// The ANSI escape sequence used before quoted strings.
    pub quoted: &'static str,
    /// The ANSI escape sequence used before escaped characters in quoted
    /// strings.
    pub escaped: &'static str,
}
