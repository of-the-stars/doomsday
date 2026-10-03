pub struct DateStamp {
    year: u16,
    month: Month,
    day: u8,
}

impl DateStamp {
    pub(crate) fn get_doomsday_code_for_century(&self) -> u16 {
        (self.year / 100) % 4
    }

    pub(crate) fn get_doomsday_for_year(&self) -> Option<Day> {
        let is_leap_year: bool = if self.year % 4 == 0 { true } else { false };

        None
    }

    // pub fn get_doomsday(&self) -> (Month, u8) {
    //     match self {
    //         Month::Jan(is_leap_year) => {
    //             if *is_leap_year {
    //                 (Month::Jan, 04)
    //             } else {
    //                 (Month::Jan, 03)
    //             }
    //         }
    //         Month::Feb(is_leap_year) => {
    //             if *is_leap_year {
    //                 (Month::Feb, 28)
    //             } else {
    //                 (Month::Feb, 29)
    //             }
    //         }
    //         Month::Mar => (Month::Mar, 14),
    //         Month::Apr => (Month::Apr, 04),
    //         Month::May => (Month::May, 09),
    //         Month::Jun => (Month::Jun, 06),
    //         Month::Jul => (Month::Jul, 11),
    //         Month::Aug => (Month::Aug, 08),
    //         Month::Sep => (Month::Sep, 05),
    //         Month::Oct => (Month::Oct, 10),
    //         Month::Nov => (Month::Nov, 07),
    //         Month::Dec => (Month::Dec, 12),
    //     }
    // }
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
    pub(crate) fn get_weekday_code(&self) -> u8 {
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

    pub(crate) fn get_day_from_code(code: u8) -> Option<Self> {
        match code % 7 {
            0 => Some(Day::Sunday),
            1 => Some(Day::Monday),
            2 => Some(Day::Tuesday),
            3 => Some(Day::Wednesday),
            4 => Some(Day::Thursday),
            5 => Some(Day::Friday),
            6 => Some(Day::Saturday),
            _ => None,
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
    pub(crate) fn get_days_in_month(&self, is_leap_year: bool) -> u8 {
        match self {
            Month::Jan => 31,
            Month::Feb => {
                if is_leap_year {
                    28
                } else {
                    29
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

    pub(crate) fn get_month_from_number(number: u8) -> Option<Month> {
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
