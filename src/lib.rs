
pub trait RelativeTime {
    /// Returns a human readable string representing the time difference
    fn to_relative(&self) -> String;
}

pub trait NegativeRelativeTime: RelativeTime {
    /// Returns a human readable string representing the time difference
    /// std::time::Duration cannot be negative, so we need a separate trait to providde past durations
    fn to_relative_in_past(&self) -> String;
}


/// Thresholds are taken from day.js
pub fn english_relative_time(secs: i64) -> String {
    if secs <= 44 {
        return "a few seconds".to_string()
    } else if secs <= 89 {
        return "a minute".to_string()
    }
    let mins = secs / 60;
    if mins <= 44 {
        return format!("{} minutes", mins);
    } else if mins <= 89 {
        return "an hour".to_string()
    }
    let hours = mins / 60;
    if hours <= 21 {
        return format!("{} hours", hours);
    } else if hours <= 35 {
        return "a day".to_string()
    }
    let days = hours / 24;
    if days <= 25 {
        return format!("{} days", days);
    } else if days <= 45 {
        return "a month".to_string()
    }
    let months = days / 30;
    if months <= 10 {
        return format!("{} months", months);
    } else if months <= 17 {
        return "a year".to_string()
    }
    let years = (months as f64 / 12.0).round() as i64;
    return format!("{:.0} years", years);
}


impl RelativeTime for std::time::Duration {
    fn to_relative(&self) -> String {
        let secs = self.as_secs();
        let relative = english_relative_time(secs as i64);
        format!("in {}", relative)
    }
}


impl NegativeRelativeTime for std::time::Duration {
    fn to_relative_in_past(&self) -> String {
        let secs = self.as_secs();
        let relative = english_relative_time(secs as i64);
        format!("{} ago", relative)
    }
}


#[cfg(feature = "chrono")]
impl RelativeTime for chrono::Duration {
    fn to_relative(&self) -> String {
        let secs = self.num_seconds();
        let relative = english_relative_time(secs.abs());
        if secs < 0 {
            format!("{} ago", relative)
        } else {
            format!("in {}", relative)
        }
    }
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> RelativeTime for chrono::DateTime<Tz> {
    fn to_relative(&self) -> String {
        let duration = self.clone().signed_duration_since(chrono::Utc::now());
        let secs = duration.num_seconds();
        println!("secs = {}", secs);
        let relative = english_relative_time(secs.abs());
        if secs < 0 {
            format!("{} ago", relative)
        } else {
            format!("in {}", relative)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::{Add, Sub};
    use std::time::Duration;
    use chrono::Utc;
    use crate::{NegativeRelativeTime, RelativeTime};

    #[test]
    fn test_std_duration() {
        let d = Duration::new(10, 0);
        assert_eq!(d.to_relative(), "in a few seconds");

        let d = Duration::new(45, 0);
        assert_eq!(d.to_relative(), "in a minute");

        let d = Duration::new(10 * 60, 0);
        assert_eq!(d.to_relative(), "in 10 minutes");

        let d = Duration::new(50 * 60, 0);
        assert_eq!(d.to_relative(), "in an hour");

        let d = Duration::new(3 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in 3 hours");

        let d = Duration::new(23 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in a day");

        let d = Duration::new(12 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in 12 days");

        let d = Duration::new(27 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in a month");

        let d = Duration::new(31 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in a month");

        let d = Duration::new(2 * 30 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in 2 months");

        let d = Duration::new(11 * 30 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in a year");

        let d = Duration::new(18 * 30 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in 2 years");

        let d = Duration::new(5 * 12 * 30 * 24 * 60 * 60, 0);
        assert_eq!(d.to_relative(), "in 5 years");

        let d = Duration::new(80, 0);
        assert_eq!(d.to_relative_in_past(), "a minute ago");
    }

    #[test]
    fn test_chrono_duration() {
        let d = chrono::Duration::seconds(10);
        assert_eq!(d.to_relative(), "in a few seconds");

        let d = chrono::Duration::minutes(-10);
        assert_eq!(d.to_relative(), "10 minutes ago");
    }

    #[test]
    fn test_chrono_datetime() {
        let d = Utc::now().sub(chrono::Duration::days(1));
        println!("{}", Utc::now());
        println!("{}", d);
        assert_eq!(d.to_relative(), "a day ago");
    }
}