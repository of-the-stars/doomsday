use std::{fmt, io::Error};

fn is_divisible_by_four(year: &u16) -> bool {
    if year % 4 == 0 { true } else { false }
}

struct DateStamp {
    year: u16,
    day_of_the_year: DayOfTheYear,
}

impl DateStamp {
    pub fn get_doomsday_code_for_century(&self) -> Result<u16, Error> {
        let century_modulo = (self.year / 100) % 4;

        match century_modulo {
            0 => Ok(2),
            1 => Ok(0),
            2 => Ok(5),
            3 => Ok(3),
            _ => Err(Error::last_os_error()),
        }
    }

    pub fn get_doomsday_code_for_year(&self) -> Result<u16, Error> {
        let century_code = self.get_doomsday_code_for_century()?;
        let leftover = self.year - ((self.year / 100) * 100);
        let a = leftover / 12;
        let b = leftover - (12 * a);
        let c = b / 4;
        let end_result = (century_code + a + b + c) % 7;

        Ok(end_result)
    }

    pub fn try_from_day_month_year_tuple(day: u8, month: u8, year: u16) -> Option<DateStamp> {
        let day_of_the_year = DayOfTheYear::try_from_day_month_year_tuple(day, month, year)?;
        Some(DateStamp {
            day_of_the_year,
            year,
        })
    }
}

pub struct DayOfTheYear {
    month: Month,
    day: u8,
}

impl DayOfTheYear {
    pub fn try_from_day_month_year_tuple(day: u8, month: u8, year: u16) -> Option<DayOfTheYear> {
        let month = Month::get_month_from_number(month)?;

        if (1..=month.max_days_in_month(is_divisible_by_four(&year))).contains(&day) {
            Some(DayOfTheYear { month, day })
        } else {
            None
        }
    }

    pub fn get_months_doomsday(&self, year: &u16) -> DayOfTheYear {
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

pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    fn get_weekday_code(&self) -> u8 {
        match self {
            Day::Sunday => 0,
            Day::Monday => 1,
            Day::Tuesday => 2,
            Day::Wednesday => 3,
            Day::Thursday => 4,
            Day::Friday => 5,
            Day::Saturday => 6,
        }
    }

    fn get_day_from_code(code: u8) -> Result<Self, Error> {
        match code % 7 {
            0 => Ok(Day::Sunday),
            1 => Ok(Day::Monday),
            2 => Ok(Day::Tuesday),
            3 => Ok(Day::Wednesday),
            4 => Ok(Day::Thursday),
            5 => Ok(Day::Friday),
            6 => Ok(Day::Saturday),
            _ => Err(Error::last_os_error()),
        }
    }
}

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
    pub(crate) fn max_days_in_month(&self, is_leap_year: bool) -> u8 {
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

    pub fn get_month_from_number(number: u8) -> Option<Month> {
        match number {
            01 => Some(Month::Jan),
            02 => Some(Month::Feb),
            03 => Some(Month::Mar),
            04 => Some(Month::Apr),
            05 => Some(Month::May),
            06 => Some(Month::Jun),
            07 => Some(Month::Jul),
            08 => Some(Month::Aug),
            09 => Some(Month::Sep),
            10 => Some(Month::Oct),
            11 => Some(Month::Nov),
            12 => Some(Month::Dec),
            _ => None,
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Month::Jan => write!(f, "Jan"),
            Month::Feb => write!(f, "Feb"),
            Month::Mar => write!(f, "Mar"),
            Month::Apr => write!(f, "Apr"),
            Month::May => write!(f, "May"),
            Month::Jun => write!(f, "Jun"),
            Month::Jul => write!(f, "Jul"),
            Month::Aug => write!(f, "Aug"),
            Month::Sep => write!(f, "Sep"),
            Month::Oct => write!(f, "Oct"),
            Month::Nov => write!(f, "Nov"),
            Month::Dec => write!(f, "Dec"),
        }
    }
}
