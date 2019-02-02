// Much of this code is based on https://github.com/OpenTTD/OpenTTD/blob/master/src/date.cpp.

use failure::Fail;
use lazy_static::*;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

const DAYS_IN_YEAR: u32 = 365;
const DAYS_IN_LEAP_YEAR: u32 = 366;
const MAX_YEAR: u32 = 5_000_000;

macro_rules! leap_years_till {
    ($year:expr) => {
        if $year == 0 {
            0
        } else {
            ($year - 1) / 4 - ($year - 1) / 100 + ($year - 1) / 400 + 1
        }
    };
}

macro_rules! days_till {
    ($year:expr) => {
        DAYS_IN_YEAR * $year + leap_years_till!($year)
    };
}

macro_rules! M {
    ($a:expr, $b:expr) => {
        (($a << 5) | $b)
    };
}
const MONTH_DATE_FROM_YEAR_DAY: [u32; 366] = [
    M!(0, 1),
    M!(0, 2),
    M!(0, 3),
    M!(0, 4),
    M!(0, 5),
    M!(0, 6),
    M!(0, 7),
    M!(0, 8),
    M!(0, 9),
    M!(0, 10),
    M!(0, 11),
    M!(0, 12),
    M!(0, 13),
    M!(0, 14),
    M!(0, 15),
    M!(0, 16),
    M!(0, 17),
    M!(0, 18),
    M!(0, 19),
    M!(0, 20),
    M!(0, 21),
    M!(0, 22),
    M!(0, 23),
    M!(0, 24),
    M!(0, 25),
    M!(0, 26),
    M!(0, 27),
    M!(0, 28),
    M!(0, 29),
    M!(0, 30),
    M!(0, 31),
    M!(1, 1),
    M!(1, 2),
    M!(1, 3),
    M!(1, 4),
    M!(1, 5),
    M!(1, 6),
    M!(1, 7),
    M!(1, 8),
    M!(1, 9),
    M!(1, 10),
    M!(1, 11),
    M!(1, 12),
    M!(1, 13),
    M!(1, 14),
    M!(1, 15),
    M!(1, 16),
    M!(1, 17),
    M!(1, 18),
    M!(1, 19),
    M!(1, 20),
    M!(1, 21),
    M!(1, 22),
    M!(1, 23),
    M!(1, 24),
    M!(1, 25),
    M!(1, 26),
    M!(1, 27),
    M!(1, 28),
    M!(1, 29),
    M!(2, 1),
    M!(2, 2),
    M!(2, 3),
    M!(2, 4),
    M!(2, 5),
    M!(2, 6),
    M!(2, 7),
    M!(2, 8),
    M!(2, 9),
    M!(2, 10),
    M!(2, 11),
    M!(2, 12),
    M!(2, 13),
    M!(2, 14),
    M!(2, 15),
    M!(2, 16),
    M!(2, 17),
    M!(2, 18),
    M!(2, 19),
    M!(2, 20),
    M!(2, 21),
    M!(2, 22),
    M!(2, 23),
    M!(2, 24),
    M!(2, 25),
    M!(2, 26),
    M!(2, 27),
    M!(2, 28),
    M!(2, 29),
    M!(2, 30),
    M!(2, 31),
    M!(3, 1),
    M!(3, 2),
    M!(3, 3),
    M!(3, 4),
    M!(3, 5),
    M!(3, 6),
    M!(3, 7),
    M!(3, 8),
    M!(3, 9),
    M!(3, 10),
    M!(3, 11),
    M!(3, 12),
    M!(3, 13),
    M!(3, 14),
    M!(3, 15),
    M!(3, 16),
    M!(3, 17),
    M!(3, 18),
    M!(3, 19),
    M!(3, 20),
    M!(3, 21),
    M!(3, 22),
    M!(3, 23),
    M!(3, 24),
    M!(3, 25),
    M!(3, 26),
    M!(3, 27),
    M!(3, 28),
    M!(3, 29),
    M!(3, 30),
    M!(4, 1),
    M!(4, 2),
    M!(4, 3),
    M!(4, 4),
    M!(4, 5),
    M!(4, 6),
    M!(4, 7),
    M!(4, 8),
    M!(4, 9),
    M!(4, 10),
    M!(4, 11),
    M!(4, 12),
    M!(4, 13),
    M!(4, 14),
    M!(4, 15),
    M!(4, 16),
    M!(4, 17),
    M!(4, 18),
    M!(4, 19),
    M!(4, 20),
    M!(4, 21),
    M!(4, 22),
    M!(4, 23),
    M!(4, 24),
    M!(4, 25),
    M!(4, 26),
    M!(4, 27),
    M!(4, 28),
    M!(4, 29),
    M!(4, 30),
    M!(4, 31),
    M!(5, 1),
    M!(5, 2),
    M!(5, 3),
    M!(5, 4),
    M!(5, 5),
    M!(5, 6),
    M!(5, 7),
    M!(5, 8),
    M!(5, 9),
    M!(5, 10),
    M!(5, 11),
    M!(5, 12),
    M!(5, 13),
    M!(5, 14),
    M!(5, 15),
    M!(5, 16),
    M!(5, 17),
    M!(5, 18),
    M!(5, 19),
    M!(5, 20),
    M!(5, 21),
    M!(5, 22),
    M!(5, 23),
    M!(5, 24),
    M!(5, 25),
    M!(5, 26),
    M!(5, 27),
    M!(5, 28),
    M!(5, 29),
    M!(5, 30),
    M!(6, 1),
    M!(6, 2),
    M!(6, 3),
    M!(6, 4),
    M!(6, 5),
    M!(6, 6),
    M!(6, 7),
    M!(6, 8),
    M!(6, 9),
    M!(6, 10),
    M!(6, 11),
    M!(6, 12),
    M!(6, 13),
    M!(6, 14),
    M!(6, 15),
    M!(6, 16),
    M!(6, 17),
    M!(6, 18),
    M!(6, 19),
    M!(6, 20),
    M!(6, 21),
    M!(6, 22),
    M!(6, 23),
    M!(6, 24),
    M!(6, 25),
    M!(6, 26),
    M!(6, 27),
    M!(6, 28),
    M!(6, 29),
    M!(6, 30),
    M!(6, 31),
    M!(7, 1),
    M!(7, 2),
    M!(7, 3),
    M!(7, 4),
    M!(7, 5),
    M!(7, 6),
    M!(7, 7),
    M!(7, 8),
    M!(7, 9),
    M!(7, 10),
    M!(7, 11),
    M!(7, 12),
    M!(7, 13),
    M!(7, 14),
    M!(7, 15),
    M!(7, 16),
    M!(7, 17),
    M!(7, 18),
    M!(7, 19),
    M!(7, 20),
    M!(7, 21),
    M!(7, 22),
    M!(7, 23),
    M!(7, 24),
    M!(7, 25),
    M!(7, 26),
    M!(7, 27),
    M!(7, 28),
    M!(7, 29),
    M!(7, 30),
    M!(7, 31),
    M!(8, 1),
    M!(8, 2),
    M!(8, 3),
    M!(8, 4),
    M!(8, 5),
    M!(8, 6),
    M!(8, 7),
    M!(8, 8),
    M!(8, 9),
    M!(8, 10),
    M!(8, 11),
    M!(8, 12),
    M!(8, 13),
    M!(8, 14),
    M!(8, 15),
    M!(8, 16),
    M!(8, 17),
    M!(8, 18),
    M!(8, 19),
    M!(8, 20),
    M!(8, 21),
    M!(8, 22),
    M!(8, 23),
    M!(8, 24),
    M!(8, 25),
    M!(8, 26),
    M!(8, 27),
    M!(8, 28),
    M!(8, 29),
    M!(8, 30),
    M!(9, 1),
    M!(9, 2),
    M!(9, 3),
    M!(9, 4),
    M!(9, 5),
    M!(9, 6),
    M!(9, 7),
    M!(9, 8),
    M!(9, 9),
    M!(9, 10),
    M!(9, 11),
    M!(9, 12),
    M!(9, 13),
    M!(9, 14),
    M!(9, 15),
    M!(9, 16),
    M!(9, 17),
    M!(9, 18),
    M!(9, 19),
    M!(9, 20),
    M!(9, 21),
    M!(9, 22),
    M!(9, 23),
    M!(9, 24),
    M!(9, 25),
    M!(9, 26),
    M!(9, 27),
    M!(9, 28),
    M!(9, 29),
    M!(9, 30),
    M!(9, 31),
    M!(10, 1),
    M!(10, 2),
    M!(10, 3),
    M!(10, 4),
    M!(10, 5),
    M!(10, 6),
    M!(10, 7),
    M!(10, 8),
    M!(10, 9),
    M!(10, 10),
    M!(10, 11),
    M!(10, 12),
    M!(10, 13),
    M!(10, 14),
    M!(10, 15),
    M!(10, 16),
    M!(10, 17),
    M!(10, 18),
    M!(10, 19),
    M!(10, 20),
    M!(10, 21),
    M!(10, 22),
    M!(10, 23),
    M!(10, 24),
    M!(10, 25),
    M!(10, 26),
    M!(10, 27),
    M!(10, 28),
    M!(10, 29),
    M!(10, 30),
    M!(11, 1),
    M!(11, 2),
    M!(11, 3),
    M!(11, 4),
    M!(11, 5),
    M!(11, 6),
    M!(11, 7),
    M!(11, 8),
    M!(11, 9),
    M!(11, 10),
    M!(11, 11),
    M!(11, 12),
    M!(11, 13),
    M!(11, 14),
    M!(11, 15),
    M!(11, 16),
    M!(11, 17),
    M!(11, 18),
    M!(11, 19),
    M!(11, 20),
    M!(11, 21),
    M!(11, 22),
    M!(11, 23),
    M!(11, 24),
    M!(11, 25),
    M!(11, 26),
    M!(11, 27),
    M!(11, 28),
    M!(11, 29),
    M!(11, 30),
    M!(11, 31),
];

const ACCUM_JAN: u32 = 0;
const ACCUM_FEB: u32 = ACCUM_JAN + 31;
const ACCUM_MAR: u32 = ACCUM_FEB + 29;
const ACCUM_APR: u32 = ACCUM_MAR + 31;
const ACCUM_MAY: u32 = ACCUM_APR + 30;
const ACCUM_JUN: u32 = ACCUM_MAY + 31;
const ACCUM_JUL: u32 = ACCUM_JUN + 30;
const ACCUM_AUG: u32 = ACCUM_JUL + 31;
const ACCUM_SEP: u32 = ACCUM_AUG + 31;
const ACCUM_OCT: u32 = ACCUM_SEP + 30;
const ACCUM_NOV: u32 = ACCUM_OCT + 31;
const ACCUM_DEC: u32 = ACCUM_NOV + 30;

fn accumulated_days_for_month(month: u32) -> Result<u32, DateError> {
    match month {
        0 => Ok(ACCUM_JAN),
        1 => Ok(ACCUM_FEB),
        2 => Ok(ACCUM_MAR),
        3 => Ok(ACCUM_APR),
        4 => Ok(ACCUM_MAY),
        5 => Ok(ACCUM_JUN),
        6 => Ok(ACCUM_JUL),
        7 => Ok(ACCUM_AUG),
        8 => Ok(ACCUM_SEP),
        9 => Ok(ACCUM_OCT),
        10 => Ok(ACCUM_NOV),
        11 => Ok(ACCUM_DEC),
        _ => Err(DateError::MonthOutOfRange { month }),
    }
}

/// The minimum day, 1 Jan 0.
const MIN_DAY: u32 = 0;

/// The number of days till the last day.
lazy_static! {
    static ref MAX_DAY: u32 = (days_till!(MAX_YEAR + 1) - 1);
}

/// An OpenTTD date.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Date(u32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum DateError {
    #[fail(display = "the date {} is out of range", date)]
    DateOutOfRange { date: u32 },
    #[fail(
        display = "the day {} is out of range in the month {} of {}",
        day, month, year
    )]
    DayOutOfRange { day: u32, month: u32, year: u32 },
    #[fail(display = "month {} is out of range", month)]
    MonthOutOfRange { month: u32 },
    #[fail(display = "year {} is out of range", year)]
    YearOutOfRange { year: u32 },
}

impl Date {
    /// Returns the OpenTTD value this date represents.
    pub fn to_openttd_date(&self) -> u32 {
        self.0
    }

    /// Returns the date this OpenTTD value represents.
    pub fn from_openttd_date(date: u32) -> Result<Date, DateError> {
        if date < MIN_DAY || date > *MAX_DAY {
            Err(DateError::DateOutOfRange { date })
        } else {
            Ok(Date(date))
        }
    }

    /// Convert a date to a year, month and day. The year will range from 0 to
    /// 5.000.000, the month from 0 to 11 and the day from 0 to 31.
    pub fn to_ymd(&self) -> (u32, u32, u32) {
        let days = self.to_openttd_date();

        /* Year determination in multiple steps to account for leap
         * years. First do the large steps, then the smaller ones.
         */

        /* There are 97 leap years in 400 years */
        let mut yr = 400 * (days / (DAYS_IN_YEAR * 400 + 97));
        let mut rem = days % (DAYS_IN_YEAR * 400 + 97);

        if rem >= DAYS_IN_YEAR * 100 + 25 {
            /* There are 25 leap years in the first 100 years after
             * every 400th year, as every 400th year is a leap year */
            yr += 100;
            rem -= DAYS_IN_YEAR * 100 + 25;

            /* There are 24 leap years in the next couple of 100 years */
            yr += 100 * (rem / (DAYS_IN_YEAR * 100 + 24));
            rem = rem % (DAYS_IN_YEAR * 100 + 24);
        }

        if !Date::is_leap_year(yr) && rem >= DAYS_IN_YEAR * 4 {
            /* The first 4 year of the century are not always a leap year */
            yr += 4;
            rem -= DAYS_IN_YEAR * 4;
        }

        /* There is 1 leap year every 4 years */
        yr += 4 * (rem / (DAYS_IN_YEAR * 4 + 1));
        rem = rem % (DAYS_IN_YEAR * 4 + 1);

        /* The last (max 3) years to account for; the first one
         * can be, but is not necessarily a leap year */
        while rem >= Date::days_in_year(yr) {
            rem -= Date::days_in_year(yr);
            yr += 1;
        }

        /* Skip the 29th of February in non-leap years */
        if !Date::is_leap_year(yr) && rem >= ACCUM_MAR - 1 {
            rem += 1;
        }

        let x = MONTH_DATE_FROM_YEAR_DAY[rem as usize];
        (yr, x >> 5, x & 0x1F)
    }

    /// Convert a year, month and day to a date. The year should be in the
    /// range 0 to 5.000.000, and the date should exist.
    pub fn from_ymd(year: u32, month: u32, day: u32) -> Result<Date, DateError> {
        if year > MAX_YEAR {
            return Err(DateError::YearOutOfRange { year });
        } else if month > 11 {
            return Err(DateError::MonthOutOfRange { month });
        } else if day == 0 || day > Date::days_in_month(year, month).unwrap() {
            return Err(DateError::DayOutOfRange { year, month, day });
        }

        /* Day-offset in a leap year */
        let mut days = accumulated_days_for_month(month).unwrap() + day - 1;

        /* Account for the missing of the 29th of February in non-leap years */
        if !Date::is_leap_year(year) && days >= ACCUM_MAR {
            days -= 1;
        }

        Ok(Date(days_till!(year) + days))
    }

    /// Returns true if the year is a leap year.
    fn is_leap_year(yr: u32) -> bool {
        yr % 4 == 0 && (yr % 100 != 0 || yr % 400 == 0)
    }

    /// Returns the number of days in the year
    fn days_in_year(yr: u32) -> u32 {
        if Date::is_leap_year(yr) {
            DAYS_IN_LEAP_YEAR
        } else {
            DAYS_IN_YEAR
        }
    }

    fn days_in_month(year: u32, month: u32) -> Result<u32, DateError> {
        match month {
            0 => Ok(31),
            1 => {
                if Date::is_leap_year(year) {
                    Ok(29)
                } else {
                    Ok(28)
                }
            }
            2 => Ok(31),
            3 => Ok(30),
            4 => Ok(31),
            5 => Ok(30),
            6 => Ok(31),
            7 => Ok(31),
            8 => Ok(30),
            9 => Ok(31),
            10 => Ok(30),
            11 => Ok(31),
            _ => Err(DateError::MonthOutOfRange { month }),
        }
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.to_openttd_date())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u32::deserialize(deserializer).and_then(|num| {
            Date::from_openttd_date(num).map_err(serde::de::Error::custom)
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use proptest::*;

    #[test]
    fn to_ymd() {
        assert_eq!(Date::from_openttd_date(0).unwrap().to_ymd(), (0, 0, 1));
    }

    #[test]
    fn from_ymd() {
        assert_eq!(
            Date::from_ymd(0, 0, 1).unwrap(),
            Date::from_openttd_date(0).unwrap()
        );
    }

    proptest! {
        /// The inner representation should not be tested, just that it converts
        /// losslessly.
        #[test]
        fn openttd_conversion(openttd_date in MIN_DAY..(days_till!(MAX_YEAR + 1) - 1)) {
            assert_eq!(
                Date::from_openttd_date(openttd_date)
                    .unwrap()
                    .to_openttd_date(),
                openttd_date
            );
        }
    }

    proptest! {
        /// Test whether ymd conversion is lossless
        #[test]
        fn ymd_conversion(openttd_date in MIN_DAY..(days_till!(MAX_YEAR + 1) - 1)) {
            let date = Date::from_openttd_date(openttd_date).unwrap();
            let (y, m, d) = date.to_ymd();
            let new_date = Date::from_ymd(y, m, d).unwrap();
            assert_eq!(new_date, date);
        }
    }
}
