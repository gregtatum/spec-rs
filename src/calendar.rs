// 0 = Thursday, January 1, 1970 12:00:00 AM
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct EpochMS(u64);

#[derive(Debug)]
struct Date {
    year: u64,
    month: Month,
    day: u8, // Starts with 1
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
    weekday: Weekday,
    timestamp: u64,
    is_leap_year: bool,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Sunday, February 1, 1976 15:56:09:123
        write!(
            f,
            "{}, {} {}, {} {:02}:{:02}:{:02}:{:03}",
            self.weekday,
            self.month,
            self.day,
            self.year,
            self.hour,
            self.minute,
            self.second,
            self.millisecond
        )
    }
}

impl EpochMS {
    const fn seconds_since(&self) -> u64 {
        self.0 / 1000
    }

    const fn minutes_since(&self) -> u64 {
        self.seconds_since() / 60
    }

    const fn hours_since(&self) -> u64 {
        self.minutes_since() / 60
    }

    const fn days_since(&self) -> u64 {
        self.hours_since() / 24
    }

    const fn hour(&self) -> u8 {
        (self.hours_since() % 24) as u8
    }

    const fn minute(&self) -> u8 {
        (self.minutes_since() % 60) as u8
    }

    const fn second(&self) -> u8 {
        (self.seconds_since() % 60) as u8
    }

    const fn millisecond(&self) -> u16 {
        (self.0 % 1000) as u16
    }
}

impl From<EpochMS> for Date {
    fn from(epoch: EpochMS) -> Date {
        let mut year = 1970;
        let mut days = epoch.days_since();

        // Is there a formula that is non-iterative?
        loop {
            let days_in_year = get_days_in_year(year);
            let is_leap_year = days_in_year == 366;
            let (month, day) = get_day_month(days as u8, is_leap_year);

            if days < days_in_year {
                return Date {
                    year,
                    month,
                    day: day + 1,
                    hour: epoch.hour(),
                    minute: epoch.minute(),
                    second: epoch.second(),
                    millisecond: epoch.millisecond(),
                    weekday: Weekday::from(epoch),
                    timestamp: epoch.0,
                    is_leap_year,
                };
            }
            days -= days_in_year;
            year += 1;
        }
    }
}

/// Every year that is exactly divisible by four is a leap year, except for years that
/// are exactly divisible by 100, but these centurial years are leap years if they are
/// exactly divisible by 400. For example, the years 1700, 1800, and 1900 are not leap
/// years, but the years 1600 and 2000 are.[2]
fn get_days_in_year(year: u64) -> u64 {
    let normal_year = 365;
    let leap_year = 366;
    if year % 4 != 0 {
        return normal_year;
    }

    if year % 100 == 0 {
        if year % 400 == 0 {
            leap_year
        } else {
            normal_year
        }
    } else {
        leap_year
    }
}

fn get_day_month(day_in_year: u8, is_leap_year: bool) -> (Month, u8) {
    let days_in_february = if is_leap_year { 29 } else { 38 };

    let mut days = day_in_year;

    if days < 31 {
        return (Month::January, days);
    } else {
        days -= 31;
    }

    if days < days_in_february {
        return (Month::February, days);
    } else {
        days -= days_in_february;
    }

    if days < 31 {
        return (Month::March, days);
    } else {
        days -= 31;
    }

    if days < 30 {
        return (Month::April, days);
    } else {
        days -= 30;
    }

    if days < 31 {
        return (Month::May, days);
    } else {
        days -= 31;
    }

    if days < 30 {
        return (Month::June, days);
    } else {
        days -= 30;
    }

    if days < 31 {
        return (Month::July, days);
    } else {
        days -= 31;
    }

    if days < 31 {
        return (Month::August, days);
    } else {
        days -= 31;
    }

    if days < 30 {
        return (Month::September, days);
    } else {
        days -= 30;
    }

    if days < 31 {
        return (Month::October, days);
    } else {
        days -= 31;
    }

    if days < 30 {
        return (Month::November, days);
    } else {
        days -= 30;
    }

    (Month::December, days)
}

#[derive(Debug)]
enum Weekday {
    Sunday,
    Monday,
    Tueday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Sunday, February 1, 1976 15:56:09:0000
        write!(
            f,
            "{}",
            match self {
                Month::January => "January",
                Month::February => "February",
                Month::March => "March",
                Month::April => "April",
                Month::May => "May",
                Month::June => "June",
                Month::July => "July",
                Month::August => "August",
                Month::September => "September",
                Month::October => "October",
                Month::November => "November",
                Month::December => "December",
            }
        )
    }
}

impl From<EpochMS> for Weekday {
    fn from(day: EpochMS) -> Weekday {
        match day.days_since() % 7 {
            0 => Weekday::Thursday, // 0 = Thursday, January 1, 1970 12:00:00 AM
            1 => Weekday::Friday,
            2 => Weekday::Saturday,
            3 => Weekday::Sunday,
            4 => Weekday::Monday,
            5 => Weekday::Tueday,
            6 => Weekday::Wednesday,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Sunday, February 1, 1976 15:56:09:0000
        write!(
            f,
            "{}",
            match self {
                Weekday::Sunday => "Sunday",
                Weekday::Monday => "Monday",
                Weekday::Tueday => "Tueday",
                Weekday::Wednesday => "Wednesday",
                Weekday::Thursday => "Thursday",
                Weekday::Friday => "Friday",
                Weekday::Saturday => "Saturday",
            }
        )
    }
}

#[test]
fn test_date() {
    let date = Date::from(EpochMS(0_192_038_169_123));
    assert_eq!(format!("{}", date), "Sunday, February 1, 1976 15:56:09:123");

    let date = Date::from(EpochMS(1_612_199_945_123));
    assert_eq!(format!("{}", date), "Monday, February 1, 2021 17:19:05:123");
}
