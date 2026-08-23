//! Date and time utilities for the CFP system
//! Provides consistent date/time handling across the application

use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
    Timelike, Utc, Weekday,
};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Timezone enum for the application
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AppTimezone {
    Utc,
    Local,
    Specific(Tz),
}

impl Default for AppTimezone {
    fn default() -> Self {
        Self::Utc
    }
}

/// Date range for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }
    
    pub fn this_month() -> Self {
        let now = Utc::now();
        let start = now.with_day(1).unwrap().with_time(NaiveTime::MIN).and_utc();
        let end = if now.month() == 12 {
            now.with_year(now.year() + 1)
                .unwrap()
                .with_month(1)
                .unwrap()
                .with_day(1)
                .unwrap()
        } else {
            now.with_month(now.month() + 1).unwrap().with_day(1).unwrap()
        };
        let end = end.with_time(NaiveTime::MIN).and_utc() - Duration::seconds(1);
        
        Self { start, end }
    }
    
    pub fn last_30_days() -> Self {
        let end = Utc::now();
        let start = end - Duration::days(30);
        Self { start, end }
    }
    
    pub fn contains(&self, datetime: &DateTime<Utc>) -> bool {
        datetime >= &self.start && datetime <= &self.end
    }
    
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}

/// CFP system date utilities
pub struct CFPDateUtils;

impl CFPDateUtils {
    /// Gets the current timestamp in UTC
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// Gets the current timestamp in local time
    pub fn now_local() -> DateTime<Local> {
        Local::now()
    }
    
    /// Formats a datetime for display
    pub fn format_display(dt: &DateTime<Utc>, format: &str) -> String {
        dt.format(format).to_string()
    }
    
    /// Formats a datetime for database storage
    pub fn format_db(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }
    
    /// Parses a date string from database format
    pub fn parse_db(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(date_str).map(|dt| dt.with_timezone(&Utc))
    }
    
    /// Checks if a date is today
    pub fn is_today(date: &NaiveDate) -> bool {
        date == &Utc::now().date_naive()
    }
    
    /// Checks if a date is in the past
    pub fn is_past(date: &NaiveDate) -> bool {
        date < &Utc::now().date_naive()
    }
    
    /// Checks if a date is in the future
    pub fn is_future(date: &NaiveDate) -> bool {
        date > &Utc::now().date_naive()
    }
    
    /// Gets the start of the day for a given date
    pub fn start_of_day(date: &NaiveDate) -> DateTime<Utc> {
        date.and_hms_opt(0, 0, 0).unwrap().and_utc()
    }
    
    /// Gets the end of the day for a given date
    pub fn end_of_day(date: &NaiveDate) -> DateTime<Utc> {
        date.and_hms_opt(23, 59, 59).unwrap().and_utc()
    }
    
    /// Calculates expiry date for verification matrix (14 days from now)
    pub fn calculate_matrix_expiry() -> NaiveDate {
        (Utc::now() + Duration::days(14)).date_naive()
    }
    
    /// Checks if verification matrix is expired
    pub fn is_matrix_expired(expiry_date: &NaiveDate) -> bool {
        expiry_date < &Utc::now().date_naive()
    }
    
    /// Calculates download limit reset date
    pub fn calculate_download_reset_date(is_donor: bool) -> DateTime<Utc> {
        let duration = if is_donor {
            Duration::days(1) // Donors: 1 day
        } else {
            Duration::weeks(1) // Non-donors: 1 week
        };
        Utc::now() + duration
    }
    
    /// Calculates voting period end date (14 days from start)
    pub fn calculate_voting_period_end(start_date: &DateTime<Utc>) -> DateTime<Utc> {
        *start_date + Duration::days(14)
    }
    
    /// Checks if voting period has ended
    pub fn is_voting_period_ended(start_date: &DateTime<Utc>) -> bool {
        let end_date = Self::calculate_voting_period_end(start_date);
        Utc::now() > end_date
    }
    
    /// Calculates committee term end date (1 year from start)
    pub fn calculate_term_end(start_date: &NaiveDate) -> NaiveDate {
        *start_date + Months::new(12)
    }
    
    /// Gets days remaining until a date
    pub fn days_until(date: &NaiveDate) -> i64 {
        let today = Utc::now().date_naive();
        (*date - today).num_days()
    }
    
    /// Gets days since a date
    pub fn days_since(date: &NaiveDate) -> i64 {
        let today = Utc::now().date_naive();
        (today - *date).num_days()
    }
    
    /// Formats relative time (e.g., "2 days ago", "in 3 hours")
    pub fn format_relative(dt: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let diff = now - *dt;
        
        if diff.num_seconds() < 60 {
            "just now".to_string()
        } else if diff.num_minutes() < 60 {
            let mins = diff.num_minutes();
            format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" })
        } else if diff.num_hours() < 24 {
            let hours = diff.num_hours();
            format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
        } else if diff.num_days() < 30 {
            let days = diff.num_days();
            format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
        } else if diff.num_days() < 365 {
            let months = diff.num_days() / 30;
            format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
        } else {
            let years = diff.num_days() / 365;
            format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
        }
    }
    
    /// Gets business days between two dates (excluding weekends)
    pub fn business_days_between(start: &NaiveDate, end: &NaiveDate) -> i64 {
        let mut business_days = 0;
        let mut current = *start;
        
        while current <= *end {
            let weekday = current.weekday();
            if weekday != Weekday::Sat && weekday != Weekday::Sun {
                business_days += 1;
            }
            current = current.succ_opt().unwrap();
        }
        
        business_days
    }
    
    /// Validates that a date range is valid (start <= end)
    pub fn validate_date_range(start: &NaiveDate, end: &NaiveDate) -> bool {
        start <= end
    }
    
    /// Gets fiscal quarter for a date
    pub fn fiscal_quarter(date: &NaiveDate) -> i32 {
        let month = date.month();
        ((month - 1) / 3 + 1) as i32
    }
    
    /// Gets fiscal year for a date (assuming fiscal year starts Jan 1)
    pub fn fiscal_year(date: &NaiveDate) -> i32 {
        date.year()
    }
}

/// Timestamp wrapper with automatic UTC conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub datetime: DateTime<Utc>,
}

impl Timestamp {
    pub fn new() -> Self {
        Self {
            datetime: Utc::now(),
        }
    }
    
    pub fn from_datetime(datetime: DateTime<Utc>) -> Self {
        Self { datetime }
    }
    
    pub fn to_naive_date(&self) -> NaiveDate {
        self.datetime.date_naive()
    }
    
    pub fn to_naive_datetime(&self) -> NaiveDateTime {
        self.datetime.naive_utc()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.datetime.format("%Y-%m-%d %H:%M:%S UTC"))
    }
}

/// Duration utilities
pub struct DurationUtils;

impl DurationUtils {
    pub fn from_days(days: i64) -> Duration {
        Duration::days(days)
    }
    
    pub fn from_hours(hours: i64) -> Duration {
        Duration::hours(hours)
    }
    
    pub fn from_minutes(minutes: i64) -> Duration {
        Duration::minutes(minutes)
    }
    
    pub fn format_human(duration: &Duration) -> String {
        let total_seconds = duration.num_seconds();
        
        if total_seconds < 60 {
            format!("{} seconds", total_seconds)
        } else if total_seconds < 3600 {
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            format!("{} minutes, {} seconds", minutes, seconds)
        } else if total_seconds < 86400 {
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            format!("{} hours, {} minutes", hours, minutes)
        } else {
            let days = total_seconds / 86400;
            let hours = (total_seconds % 86400) / 3600;
            format!("{} days, {} hours", days, hours)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    #[test]
    fn test_date_range() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 31, 23, 59, 59).unwrap();
        let range = DateRange::new(start, end);
        
        let middle = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        assert!(range.contains(&middle));
        
        let before = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        assert!(!range.contains(&before));
    }
    
    #[test]
    fn test_cfp_date_utils() {
        let today = Utc::now().date_naive();
        assert!(CFPDateUtils::is_today(&today));
        
        let yesterday = today - Duration::days(1);
        assert!(CFPDateUtils::is_past(&yesterday));
        
        let tomorrow = today + Duration::days(1);
        assert!(CFPDateUtils::is_future(&tomorrow));
        
        let expiry = CFPDateUtils::calculate_matrix_expiry();
        assert_eq!(CFPDateUtils::days_until(&expiry), 14);
        
        let start_date = Utc::now();
        let voting_end = CFPDateUtils::calculate_voting_period_end(&start_date);
        assert_eq!((voting_end - start_date).num_days(), 14);
    }
    
    #[test]
    fn test_relative_format() {
        let now = Utc::now();
        let five_minutes_ago = now - Duration::minutes(5);
        let formatted = CFPDateUtils::format_relative(&five_minutes_ago);
        assert!(formatted.contains("minute"));
        
        let two_days_ago = now - Duration::days(2);
        let formatted = CFPDateUtils::format_relative(&two_days_ago);
        assert!(formatted.contains("day"));
    }
    
    #[test]
    fn test_business_days() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Monday
        let friday = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(); // Friday
        
        let business_days = CFPDateUtils::business_days_between(&monday, &friday);
        assert_eq!(business_days, 5);
        
        let next_monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let business_days = CFPDateUtils::business_days_between(&friday, &next_monday);
        assert_eq!(business_days, 1); // Only Monday (Sat/Sun excluded)
    }
}
