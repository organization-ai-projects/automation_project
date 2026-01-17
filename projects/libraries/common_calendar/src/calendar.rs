// projects/libraries/common_calendar/src/calendar.rs
use chrono::NaiveDate;

/// Represents a simple calendar utility.
pub struct Calendar;

impl Calendar {
    /// Checks if a given year is a leap year.
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a given month of a specific year.
    pub fn days_in_month(year: i32, month: u32) -> Option<u32> {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
            4 | 6 | 9 | 11 => Some(30),
            2 => {
                if Self::is_leap_year(year) {
                    Some(29)
                } else {
                    Some(28)
                }
            }
            _ => None,
        }
    }

    /// Parses a date string in the format `YYYY-MM-DD`.
    pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
    }
}
