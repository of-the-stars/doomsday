use std::{error, fmt, io, num, process, result, time};

/// A `Result` type alias, returning [`CalError`] variants based off of the errors that may occur
pub type Result<T> = result::Result<T, CalError>;

pub(crate) struct GuessTime(time::Duration);

impl GuessTime {
    pub(crate) fn new(duration: time::Duration) -> Self {
        Self(duration)
    }
}

impl fmt::Display for GuessTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s {}ms", self.0.as_secs(), self.0.subsec_millis())
    }
}

/// Most of these errors should technically never be encountered,
/// but I didn't want to write code that wasn't idiomatic
/// and I sure as hell wasn't about to ignore the type system.
///
/// I <3 the Rust type system.
///
/// I'd rather do all this than have to wade into the muck that
/// is unsafe code.
///
/// Instead of making a safe wrapper around unsafe code, it's
/// better to just use the safe tools already provided by the
/// language.
#[derive(Debug)]
pub enum CalError {
    /// If the numbers given simply don't give a valid date
    InvalidDate,
    /// If an IO operation failed
    IoError(io::Error),
    /// If parsing an integer from a string failed
    IntError(num::ParseIntError),
    /// If a module operation on unsigned bound integers miraculously failed
    Modulo,
    /// If a value guaranteed to be in `(1..=31)` were miraculously not to be in that range
    SignConversion,
    /// If a value guaranteed to be less than 100 were to miraculously not fit in a [`u8`]
    SizeConversion,
    /// If parsing a command line argument fails
    ArgError,
}

impl error::Error for CalError {}

impl process::Termination for CalError {
    fn report(self) -> process::ExitCode {
        println!("{self}");
        process::ExitCode::FAILURE
    }
}

impl From<io::Error> for CalError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<num::ParseIntError> for CalError {
    fn from(value: num::ParseIntError) -> Self {
        Self::IntError(value)
    }
}

impl fmt::Display for CalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalError::InvalidDate => {
                write!(f, "The numbers given did not transform to a valid date")
            }
            CalError::IntError(e) => write!(
                f,
                "An error was encountered while parsing an integer: {}",
                e
            ),
            CalError::IoError(e) => write!(
                f,
                "An error was encountered while performing an IO operation: {}",
                e
            ),
            CalError::Modulo => write!(
                f,
                "Got a value outside the mathematical range of the modulo operation somehow"
            ),
            CalError::SignConversion => write!(
                f,
                "Difference >= 1 should be true for all differences; somehow that didn't happen"
            ),
            CalError::SizeConversion => write!(
                f,
                "Value <= 100 should be true for all values; somehow that got violated"
            ),
            CalError::ArgError => write!(f, "Invalid command arguments"),
        }
    }
}
