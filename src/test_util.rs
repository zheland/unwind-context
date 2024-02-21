use core::fmt::{
    Arguments as FmtArguments, Debug, Error as FmtError, Formatter, Result as FmtResult,
    Write as FmtWrite,
};
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::sync::mpsc;

#[derive(Debug)]
pub struct FixedBufWriter<'a> {
    buffer: &'a mut [u8],
    used: usize,
}

impl<'a> FixedBufWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, used: 0 }
    }

    pub fn into_str(self) -> &'a str {
        core::str::from_utf8(&self.buffer[0..self.used]).expect("unexpected UTF8 error")
    }
}

impl FmtWrite for FixedBufWriter<'_> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        let from = self.used;
        let until = from.checked_add(s.len()).ok_or(FmtError)?;
        self.buffer
            .get_mut(from..until)
            .ok_or(FmtError)?
            .copy_from_slice(s.as_bytes());
        self.used = until;
        Ok(())
    }
}

pub fn buf_fmt<'a>(buffer: &'a mut [u8], args: FmtArguments<'_>) -> Result<&'a str, FmtError> {
    let mut writer = FixedBufWriter::new(buffer);
    core::fmt::write(&mut writer, args)?;
    Ok(writer.into_str())
}

pub fn debug_fmt<'a, T>(buffer: &'a mut [u8], value: &T) -> Result<&'a str, FmtError>
where
    T: Debug,
{
    buf_fmt(buffer, format_args!("{value:?}"))
}

#[derive(Clone)]
pub struct TransparentDebug(pub &'static str);

impl Debug for TransparentDebug {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.0)
    }
}

pub trait PatternMatcher<'a> {
    fn expect_str(&mut self, value: &str) -> Result<(), PatternMatcherError>;
    fn read_until(&mut self, pat: &str) -> Result<&'a str, PatternMatcherError>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PatternMatcherError;

impl<'a> PatternMatcher<'a> for &'a str {
    fn expect_str(&mut self, value: &str) -> Result<(), PatternMatcherError> {
        if let Some(rest) = self.strip_prefix(value) {
            *self = rest;
            Ok(())
        } else {
            Err(PatternMatcherError)
        }
    }

    fn read_until(&mut self, pat: &str) -> Result<&'a str, PatternMatcherError> {
        if let Some((prefix, suffix)) = self.split_once(pat) {
            *self = suffix;
            Ok(prefix)
        } else {
            Err(PatternMatcherError)
        }
    }
}

#[cfg(feature = "std")]
#[allow(clippy::arithmetic_side_effects)]
pub fn collect_string_from_recv(recv: &mpsc::Receiver<String>) -> String {
    let mut data = String::new();
    while let Ok(value) = recv.try_recv() {
        data += &value;
    }
    data
}

#[test]
fn test_pattern_matcher() {
    let value = "foo bar baz";
    let value_ref = &mut &*value;
    assert_eq!(value_ref.expect_str("foo"), Ok(()));
    assert_eq!(value_ref.expect_str(" b"), Ok(()));
    assert_eq!(value_ref.expect_str("art"), Err(PatternMatcherError));
    assert_eq!(value_ref.expect_str("ar"), Ok(()));
    assert_eq!(value_ref.expect_str(" baz"), Ok(()));

    let value = "foo bar baz";
    let value_ref = &mut &*value;
    assert_eq!(value_ref.read_until(" "), Ok("foo"));
    assert_eq!(value_ref.read_until("a"), Ok("b"));
    assert_eq!(value_ref.read_until("r"), Ok(""));
    assert_eq!(value_ref.read_until("q"), Err(PatternMatcherError));
    assert_eq!(value_ref.read_until("a"), Ok(" b"));
    assert_eq!(value_ref.read_until("z"), Ok(""));
}

#[test]
fn test_no_panic_on_buffer_overflow() {
    let mut buffer = [0; 10];
    let mut writer = FixedBufWriter::new(&mut buffer);
    assert_eq!(writer.write_str("0123456789"), Ok(()));

    let mut buffer = [0; 9];
    let mut writer = FixedBufWriter::new(&mut buffer);
    assert_eq!(writer.write_str("0123456789"), Err(FmtError));

    let mut writer = FixedBufWriter::new(&mut []);

    // Emulate an inconsistent state with a very high value of buffer used.
    writer.used = usize::MAX - 9;

    assert_eq!(writer.write_str("0123456789"), Err(FmtError));
}
