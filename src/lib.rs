use std::{
    fmt,
    io::{self, prelude::*},
    num, process, str,
};

/// A `Result` type alias, returning [`CalError`] variants based off of the errors that may occur
pub type Result<T> = std::result::Result<T, CalError>;

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
    /// If a value guaranteed to be in (1..=31) were miraculously not to be in that range
    SignConversion,
    /// If a value guaranteed to be less than 100 were to miraculously not fit in a u8
    SizeConversion,
}

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
        }
    }
}

fn is_divisible_by_four(year: &u16) -> bool {
    if year % 4 == 0 { true } else { false }
}

/// Represents a date from the Gregorian calendar.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct DateStamp {
    /// The year
    year: u16,
    /// A combination of the month and day
    day_of_the_year: DayOfTheYear,
}

impl DateStamp {
    /// Gets the base doomsday code for the date stamp's century
    fn get_doomsday_code_for_century(&self) -> Result<u8> {
        let century_modulo = (self.year / 100) % 4;

        match century_modulo {
            0 => Ok(2),
            1 => Ok(0),
            2 => Ok(5),
            3 => Ok(3),
            _ => Err(CalError::Modulo),
        }
    }

    /// Gets the doomsday code for the date stamp's year
    fn get_doomsday_code_for_year(&self) -> Result<u8> {
        let century_code = self.get_doomsday_code_for_century()?;
        let leftover = self.year - ((self.year / 100) * 100);
        let leftover: u8 = leftover.try_into().or(Err(CalError::SizeConversion))?;
        let a = leftover / 12;
        let b = leftover - (12 * a);
        let c = b / 4;
        let end_result = (century_code + a + b + c) % 7;

        Ok(end_result)
    }

    /// Gets the doomsday code for the date stamp's day
    fn get_doomsday_code_for_day(&self) -> Result<u8> {
        let doomsday_code: i16 = self
            .get_doomsday_code_for_year()?
            .try_into()
            .or(Err(CalError::SignConversion))?;

        let diff = self
            .day_of_the_year
            .get_difference_from_doomsday(&self.year);

        let code: u8 = (doomsday_code + diff)
            .try_into()
            .or(Err(CalError::SignConversion))?;

        Ok(code)
    }

    /// Returns the day of the week the date stamp falls on
    pub fn get_day_of_the_week(&self) -> Result<DayOfTheWeek> {
        let code = self.get_doomsday_code_for_day()?;

        DayOfTheWeek::get_day_from_code(code)
    }

    /// Creates a [`DateStamp`] from a day, month, year [`(u8, u8, 16)`] tuple.
    /// If it's not a valid date, it returns a [`CalError::InvalidDate`].
    ///
    /// # Example
    /// ```rust
    /// let tuple: (u8, u8, u16) = (29, 02, 2000);
    /// let date_stamp = DateStamp {
    ///     year: 2000,
    ///     day_of_the_year: DayOfTheYear {
    ///         month: Month:: Feb,
    ///         day: 29
    ///     },
    /// };
    /// let tuple = DateStamp::try_from_day_month_year_tuple(tuple);
    ///
    /// assert!(date_stamp == tuple);
    /// ```
    pub fn try_from_tuple((day, month, year): (u8, u8, u16)) -> Result<DateStamp> {
        let day_of_the_year = DayOfTheYear::try_from_tuple((day, month, year))?;

        Ok(DateStamp {
            day_of_the_year,
            year,
        })
    }

    pub fn try_from_day_month_year(day: u8, month: Month, year: u16) -> Result<DateStamp> {
        let day_of_the_year = DayOfTheYear::try_from_day_month_year(day, month, year)?;

        Ok(DateStamp {
            day_of_the_year,
            year,
        })
    }

    pub fn try_from_input() -> Result<DateStamp> {
        let mut day_buffer = String::new();
        let mut month_buffer = String::new();
        let mut year_buffer = String::new();

        let mut day: Result<u8> = Err(CalError::InvalidDate);
        let mut month: Result<Month> = Err(CalError::InvalidDate);
        let mut year: Result<u16> = Err(CalError::InvalidDate);

        while let Err(_) = day {
            print!("Enter a day: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut day_buffer)?;
            day = day_buffer.trim().parse().map_err(|e| CalError::IntError(e));
            day_buffer.clear();
        }

        while let Err(_) = month {
            print!("Enter a month: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut month_buffer)?;
            month = month_buffer.trim().parse();
            month_buffer.clear();
        }

        while let Err(_) = year {
            print!("Enter a year: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut year_buffer)?;
            year = year_buffer
                .trim()
                .parse()
                .map_err(|e| CalError::IntError(e));
            year_buffer.clear();
        }

        let day = day?;
        let month = month?;
        let year = year?;

        let date_stamp = DateStamp::try_from_day_month_year(day, month, year)?;

        Ok(date_stamp)
    }
}

impl fmt::Display for DateStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day_of_the_year, self.year)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct DayOfTheYear {
    month: Month,
    day: u8,
}

impl DayOfTheYear {
    fn try_from_tuple((day, month, year): (u8, u8, u16)) -> Result<DayOfTheYear> {
        let month = Month::try_from_number(month)?;
        let days_in_month = month.max_days_in_month(is_divisible_by_four(&year));

        if (1..=days_in_month).contains(&day) {
            Ok(DayOfTheYear { month, day })
        } else {
            Err(CalError::InvalidDate)
        }
    }

    pub fn try_from_day_month_year(day: u8, month: Month, year: u16) -> Result<DayOfTheYear> {
        let days_in_month = month.max_days_in_month(is_divisible_by_four(&year));

        if (1..=days_in_month).contains(&day) {
            Ok(DayOfTheYear { month, day })
        } else {
            Err(CalError::InvalidDate)
        }
    }

    fn get_difference_from_doomsday(&self, year: &u16) -> i16 {
        let closest_doomsday: i16 = self.get_months_doomsday(year).day.into();
        let day: i16 = self.day.into();

        day - closest_doomsday
    }

    fn get_months_doomsday(&self, year: &u16) -> DayOfTheYear {
        let is_leap_year = is_divisible_by_four(year);

        match self.month {
            Month::Jan => {
                if is_leap_year {
                    DayOfTheYear {
                        month: Month::Jan,
                        day: 04,
                    }
                } else {
                    DayOfTheYear {
                        month: Month::Jan,
                        day: 03,
                    }
                }
            }
            Month::Feb => {
                if is_leap_year {
                    DayOfTheYear {
                        month: Month::Feb,
                        day: 29,
                    }
                } else {
                    DayOfTheYear {
                        month: Month::Feb,
                        day: 28,
                    }
                }
            }
            Month::Mar => DayOfTheYear {
                month: Month::Mar,
                day: 14,
            },
            Month::Apr => DayOfTheYear {
                month: Month::Apr,
                day: 04,
            },
            Month::May => DayOfTheYear {
                month: Month::May,
                day: 09,
            },
            Month::Jun => DayOfTheYear {
                month: Month::Jun,
                day: 06,
            },
            Month::Jul => DayOfTheYear {
                month: Month::Jul,
                day: 11,
            },
            Month::Aug => DayOfTheYear {
                month: Month::Aug,
                day: 08,
            },
            Month::Sep => DayOfTheYear {
                month: Month::Sep,
                day: 05,
            },
            Month::Oct => DayOfTheYear {
                month: Month::Oct,
                day: 10,
            },
            Month::Nov => DayOfTheYear {
                month: Month::Nov,
                day: 07,
            },
            Month::Dec => DayOfTheYear {
                month: Month::Dec,
                day: 12,
            },
        }
    }
}

impl fmt::Display for DayOfTheYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day, self.month)
    }
}

/// Represents each day of the week
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DayOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl DayOfTheWeek {
    #[allow(dead_code)]
    fn get_weekday_code(&self) -> u8 {
        match self {
            DayOfTheWeek::Sunday => 0,
            DayOfTheWeek::Monday => 1,
            DayOfTheWeek::Tuesday => 2,
            DayOfTheWeek::Wednesday => 3,
            DayOfTheWeek::Thursday => 4,
            DayOfTheWeek::Friday => 5,
            DayOfTheWeek::Saturday => 6,
        }
    }

    fn get_day_from_code(code: u8) -> Result<Self> {
        match code % 7 {
            0 => Ok(DayOfTheWeek::Sunday),
            1 => Ok(DayOfTheWeek::Monday),
            2 => Ok(DayOfTheWeek::Tuesday),
            3 => Ok(DayOfTheWeek::Wednesday),
            4 => Ok(DayOfTheWeek::Thursday),
            5 => Ok(DayOfTheWeek::Friday),
            6 => Ok(DayOfTheWeek::Saturday),
            _ => Err(CalError::Modulo),
        }
    }
}

impl fmt::Display for DayOfTheWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayOfTheWeek::Sunday => write!(f, "Sunday"),
            DayOfTheWeek::Monday => write!(f, "Monday"),
            DayOfTheWeek::Tuesday => write!(f, "Tuesday"),
            DayOfTheWeek::Wednesday => write!(f, "Wednesday"),
            DayOfTheWeek::Thursday => write!(f, "Thursday"),
            DayOfTheWeek::Friday => write!(f, "Friday"),
            DayOfTheWeek::Saturday => write!(f, "Saturday"),
        }
    }
}

/// Represents each month of the year
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    fn max_days_in_month(&self, is_leap_year: bool) -> u8 {
        match self {
            Month::Jan => 31,
            Month::Feb => {
                if is_leap_year {
                    29
                } else {
                    28
                }
            }
            Month::Mar => 31,
            Month::Apr => 30,
            Month::May => 31,
            Month::Jun => 30,
            Month::Jul => 31,
            Month::Aug => 31,
            Month::Sep => 30,
            Month::Oct => 31,
            Month::Nov => 30,
            Month::Dec => 31,
        }
    }

    fn try_from_number(number: u8) -> Result<Month> {
        match number {
            01 => Ok(Month::Jan),
            02 => Ok(Month::Feb),
            03 => Ok(Month::Mar),
            04 => Ok(Month::Apr),
            05 => Ok(Month::May),
            06 => Ok(Month::Jun),
            07 => Ok(Month::Jul),
            08 => Ok(Month::Aug),
            09 => Ok(Month::Sep),
            10 => Ok(Month::Oct),
            11 => Ok(Month::Nov),
            12 => Ok(Month::Dec),
            _ => Err(CalError::InvalidDate),
        }
    }

    fn try_from_str(s: &str) -> Result<Month> {
        match s {
            "Jan" => Ok(Month::Jan),
            "January" => Ok(Month::Jan),
            "Feb" => Ok(Month::Feb),
            "February" => Ok(Month::Feb),
            "Mar" => Ok(Month::Mar),
            "March" => Ok(Month::Mar),
            "Apr" => Ok(Month::Apr),
            "April" => Ok(Month::Apr),
            "May" => Ok(Month::May),
            "Jun" => Ok(Month::Jun),
            "June" => Ok(Month::Jun),
            "Jul" => Ok(Month::Jul),
            "July" => Ok(Month::Jul),
            "Aug" => Ok(Month::Aug),
            "August" => Ok(Month::Aug),
            "Sep" => Ok(Month::Sep),
            "September" => Ok(Month::Sep),
            "Oct" => Ok(Month::Oct),
            "October" => Ok(Month::Oct),
            "Nov" => Ok(Month::Nov),
            "November" => Ok(Month::Nov),
            "Dec" => Ok(Month::Dec),
            "December" => Ok(Month::Dec),
            "jan" => Ok(Month::Jan),
            "january" => Ok(Month::Jan),
            "feb" => Ok(Month::Feb),
            "february" => Ok(Month::Feb),
            "mar" => Ok(Month::Mar),
            "march" => Ok(Month::Mar),
            "apr" => Ok(Month::Apr),
            "april" => Ok(Month::Apr),
            "may" => Ok(Month::May),
            "jun" => Ok(Month::Jun),
            "june" => Ok(Month::Jun),
            "jul" => Ok(Month::Jul),
            "july" => Ok(Month::Jul),
            "aug" => Ok(Month::Aug),
            "august" => Ok(Month::Aug),
            "sep" => Ok(Month::Sep),
            "september" => Ok(Month::Sep),
            "oct" => Ok(Month::Oct),
            "october" => Ok(Month::Oct),
            "nov" => Ok(Month::Nov),
            "november" => Ok(Month::Nov),
            "dec" => Ok(Month::Dec),
            "december" => Ok(Month::Dec),
            _ => Err(CalError::InvalidDate),
        }
    }
}

impl str::FromStr for Month {
    type Err = CalError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let s = s.trim();

        match Month::try_from_str(s) {
            Ok(month) => Ok(month),
            Err(_) => match Month::try_from_number(s.parse::<u8>()?) {
                Ok(month) => Ok(month),
                Err(err) => Err(err),
            },
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Month::Jan => write!(f, "January"),
            Month::Feb => write!(f, "February"),
            Month::Mar => write!(f, "March"),
            Month::Apr => write!(f, "April"),
            Month::May => write!(f, "May"),
            Month::Jun => write!(f, "June"),
            Month::Jul => write!(f, "July"),
            Month::Aug => write!(f, "August"),
            Month::Sep => write!(f, "September"),
            Month::Oct => write!(f, "October"),
            Month::Nov => write!(f, "November"),
            Month::Dec => write!(f, "December"),
        }
    }
}
