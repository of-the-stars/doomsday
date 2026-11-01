use crate::helper_types::{CalError, Result};
use std::{fmt, str};

/// Checks if the given year is a leap year. In the Gregorian calendar, leap years are years that are divisible by four and not
/// divisible by one hundred unless they're also divisible by four hundred
fn is_leap_year(year: &u16) -> bool {
    let divisible_by_four: bool = year % 4 == 0;
    let divisible_by_one_hundred: bool = year % 100 == 0;
    let divisible_by_four_hundred: bool = year % 400 == 0;

    divisible_by_four && (!divisible_by_one_hundred || divisible_by_four_hundred)
}

/// Represents a date from the Gregorian calendar.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) struct DateStamp {
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
        let century = (self.year / 100) * 100;

        let leftover = self.year - century;
        let leftover: u8 = leftover.try_into().or(Err(CalError::SizeConversion))?;

        let a = leftover / 12;
        let b = leftover - (12 * a);
        let c = b / 4;

        let end_result = (century_code + a + b + c) % 7;

        Ok(end_result)
    }

    /// Gets the doomsday code for the date stamp's day
    fn get_doomsday_code_for_day(&self) -> Result<u8> {
        let doomsday_code: i16 = self.get_doomsday_code_for_year()?.into();

        let diff = self
            .day_of_the_year
            .get_difference_from_doomsday(&self.year);

        let mut code = doomsday_code + diff;

        while code < 0 {
            code += 7;
        }

        let code: u8 = code
            .rem_euclid(7)
            .try_into()
            .or(Err(CalError::SignConversion))?;

        Ok(code)
    }

    /// Returns the day of the week the date stamp falls on
    pub(crate) fn get_day_of_the_week(&self) -> Result<DayOfTheWeek> {
        let code = self.get_doomsday_code_for_day()?;

        DayOfTheWeek::try_from_code(code)
    }

    /// Creates a [`DateStamp`] from a day, month, year [`(u8, u8, 16)`] tuple.
    /// If it's not a valid date, it returns a [`CalError::InvalidDate`].
    pub(crate) fn try_from_tuple((day, month, year): (u8, u8, u16)) -> Result<DateStamp> {
        let day_of_the_year = DayOfTheYear::try_from_tuple((day, month, year))?;

        Ok(DateStamp {
            day_of_the_year,
            year,
        })
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
        let days_in_month = month.max_days_in_month(is_leap_year(&year));

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
        let is_leap_year = is_leap_year(year);

        let doomsday = match self.month {
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
        };

        doomsday
    }
}

impl fmt::Display for DayOfTheYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day, self.month)
    }
}

/// Represents each day of the week
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum DayOfTheWeek {
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

    fn try_from_code(code: u8) -> Result<Self> {
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

    fn try_from_str(s: &str) -> Result<Self> {
        match s {
            "Sunday" => Ok(DayOfTheWeek::Sunday),
            "Monday" => Ok(DayOfTheWeek::Monday),
            "Tuesday" => Ok(DayOfTheWeek::Tuesday),
            "Wednesday" => Ok(DayOfTheWeek::Wednesday),
            "Thursday" => Ok(DayOfTheWeek::Thursday),
            "Friday" => Ok(DayOfTheWeek::Friday),
            "Saturday" => Ok(DayOfTheWeek::Saturday),
            "Sun" => Ok(DayOfTheWeek::Sunday),
            "Mon" => Ok(DayOfTheWeek::Monday),
            "Tue" => Ok(DayOfTheWeek::Tuesday),
            "Wed" => Ok(DayOfTheWeek::Wednesday),
            "Thu" => Ok(DayOfTheWeek::Thursday),
            "Fri" => Ok(DayOfTheWeek::Friday),
            "Sat" => Ok(DayOfTheWeek::Saturday),
            "sunday" => Ok(DayOfTheWeek::Sunday),
            "monday" => Ok(DayOfTheWeek::Monday),
            "tuesday" => Ok(DayOfTheWeek::Tuesday),
            "wednesday" => Ok(DayOfTheWeek::Wednesday),
            "thursday" => Ok(DayOfTheWeek::Thursday),
            "friday" => Ok(DayOfTheWeek::Friday),
            "saturday" => Ok(DayOfTheWeek::Saturday),
            "sun" => Ok(DayOfTheWeek::Sunday),
            "mon" => Ok(DayOfTheWeek::Monday),
            "tue" => Ok(DayOfTheWeek::Tuesday),
            "wed" => Ok(DayOfTheWeek::Wednesday),
            "thu" => Ok(DayOfTheWeek::Thursday),
            "fri" => Ok(DayOfTheWeek::Friday),
            "sat" => Ok(DayOfTheWeek::Saturday),
            _ => Err(CalError::InvalidDate),
        }
    }
}

impl str::FromStr for DayOfTheWeek {
    type Err = CalError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let s = s.trim();

        match DayOfTheWeek::try_from_str(s) {
            Ok(day) => Ok(day),
            Err(_) => match DayOfTheWeek::try_from_code(s.parse::<u8>()?) {
                Ok(day) => Ok(day),
                Err(err) => Err(err),
            },
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
enum Month {
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
