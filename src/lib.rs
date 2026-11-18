use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Timelike, Utc, Weekday};

fn epoch() -> NaiveDateTime {
    DateTime::from_timestamp(0, 0).unwrap().naive_utc()
}

pub fn add_days(dt: &NaiveDateTime, days: i64) -> NaiveDateTime {
    *dt + Duration::days(days)
}

pub fn add_hours(dt: &NaiveDateTime, hours: i64) -> NaiveDateTime {
    *dt + Duration::hours(hours)
}

pub fn add_minutes(dt: &NaiveDateTime, minutes: i64) -> NaiveDateTime {
    *dt + Duration::minutes(minutes)
}

pub fn add_seconds(dt: &NaiveDateTime, seconds: i64) -> NaiveDateTime {
    *dt + Duration::seconds(seconds)
}

pub fn add_months(dt: &NaiveDateTime, months: i32) -> NaiveDateTime {
    let total = dt.year() * 12 + (dt.month0() as i32) + months;
    let year = total.div_euclid(12);
    let month0 = total.rem_euclid(12);
    let day = dt.day().min(days_in_month(year, month0 as u32 + 1));
    epoch()
        .with_year(year)
        .unwrap()
        .with_month0(month0 as u32)
        .unwrap()
        .with_day(day)
        .unwrap()
        .with_hour(dt.hour())
        .unwrap()
        .with_minute(dt.minute())
        .unwrap()
        .with_second(dt.second())
        .unwrap()
}

pub fn add_years(dt: &NaiveDateTime, years: i32) -> NaiveDateTime {
    add_months(dt, years * 12)
}

pub fn start_of_day(dt: &NaiveDateTime) -> NaiveDateTime {
    dt.date().and_hms_opt(0, 0, 0).unwrap()
}

pub fn end_of_day(dt: &NaiveDateTime) -> NaiveDateTime {
    dt.date().and_hms_opt(23, 59, 59).unwrap()
}

pub fn start_of_week(dt: &NaiveDateTime) -> NaiveDateTime {
    let days_since_monday = dt.weekday().num_days_from_monday() as i64;
    add_days(&start_of_day(dt), -days_since_monday)
}

pub fn end_of_week(dt: &NaiveDateTime) -> NaiveDateTime {
    end_of_day(&add_days(&start_of_week(dt), 6))
}

pub fn start_of_month(dt: &NaiveDateTime) -> NaiveDateTime {
    dt.date().with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap()
}

pub fn end_of_month(dt: &NaiveDateTime) -> NaiveDateTime {
    end_of_day(&add_days(&add_months(&start_of_month(dt), 1), -1))
}

pub fn start_of_year(dt: &NaiveDateTime) -> NaiveDateTime {
    dt.date()
        .with_month(1)
        .unwrap()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
}

pub fn end_of_year(dt: &NaiveDateTime) -> NaiveDateTime {
    end_of_day(&add_years(&start_of_year(dt), 1))
}

pub fn is_same_day(a: &NaiveDateTime, b: &NaiveDateTime) -> bool {
    a.date() == b.date()
}

pub fn is_same_month(a: &NaiveDateTime, b: &NaiveDateTime) -> bool {
    a.year() == b.year() && a.month() == b.month()
}

pub fn is_same_year(a: &NaiveDateTime, b: &NaiveDateTime) -> bool {
    a.year() == b.year()
}

pub fn is_weekend(dt: &NaiveDateTime) -> bool {
    matches!(dt.weekday(), Weekday::Sat | Weekday::Sun)
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    let probe = epoch()
        .with_year(year)
        .unwrap()
        .with_month(month)
        .unwrap()
        .with_day(1)
        .unwrap();
    add_months(&probe, 1)
        .signed_duration_since(probe)
        .num_days() as u32
}

pub fn difference_in_days(a: &NaiveDateTime, b: &NaiveDateTime) -> i64 {
    a.signed_duration_since(*b).num_days()
}

pub fn difference_in_hours(a: &NaiveDateTime, b: &NaiveDateTime) -> i64 {
    a.signed_duration_since(*b).num_hours()
}

pub fn difference_in_minutes(a: &NaiveDateTime, b: &NaiveDateTime) -> i64 {
    a.signed_duration_since(*b).num_minutes()
}

pub fn difference_in_seconds(a: &NaiveDateTime, b: &NaiveDateTime) -> i64 {
    a.signed_duration_since(*b).num_seconds()
}

pub fn quarter(dt: &NaiveDateTime) -> u32 {
    (dt.month0() / 3) + 1
}

pub fn format_iso(dt: &NaiveDateTime) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn format_date(dt: &NaiveDateTime) -> String {
    dt.format("%Y-%m-%d").to_string()
}

pub fn format_time(dt: &NaiveDateTime) -> String {
    dt.format("%H:%M:%S").to_string()
}

#[allow(dead_code)]
fn now_utc() -> NaiveDateTime {
    Utc::now().naive_utc()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dt(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> NaiveDateTime {
        DateTime::from_timestamp(0, 0)
            .unwrap()
            .naive_utc()
            .with_year(y)
            .unwrap()
            .with_month0(mo - 1)
            .unwrap()
            .with_day(d)
            .unwrap()
            .with_hour(h)
            .unwrap()
            .with_minute(mi)
            .unwrap()
            .with_second(s)
            .unwrap()
    }

    #[test]
    fn add_arithmetic() {
        let base = dt(2026, 1, 31, 12, 0, 0);
        let next = add_months(&base, 1);
        assert_eq!((next.year(), next.month(), next.day()), (2026, 2, 28));
        let prev = add_months(&base, -1);
        assert_eq!(prev.month(), 12);
        assert_eq!(add_days(&base, 1).day(), 1);
        assert_eq!(add_hours(&base, -13).day(), 30);
    }

    #[test]
    fn boundaries() {
        let mid = dt(2026, 3, 15, 13, 45, 30);
        assert_eq!(start_of_day(&mid), dt(2026, 3, 15, 0, 0, 0));
        assert_eq!(end_of_month(&mid).day(), 31);
        assert_eq!(start_of_year(&mid).month(), 1);
        assert_eq!(end_of_year(&mid).month(), 12);
        assert_eq!(end_of_day(&mid).hour(), 23);
        let monday = start_of_week(&mid);
        assert_eq!(monday.weekday(), Weekday::Mon);
        assert_eq!(end_of_week(&mid).weekday(), Weekday::Sun);
    }

    #[test]
    fn predicates_and_diffs() {
        let a = dt(2026, 1, 1, 10, 0, 0);
        let b = dt(2026, 1, 1, 22, 30, 0);
        assert!(is_same_day(&a, &b));
        assert!(is_same_month(&a, &b));
        assert!(is_same_year(&a, &b));
        assert!(is_leap_year(2028));
        assert!(!is_leap_year(2026));
        assert!(is_weekend(&dt(2026, 11, 21, 0, 0, 0)));
        assert_eq!(days_in_month(2026, 2), 28);
        assert_eq!(days_in_month(2028, 2), 29);
        assert_eq!(difference_in_days(&b, &a), 0);
        assert_eq!(difference_in_hours(&b, &a), 12);
        assert_eq!(quarter(&mid), 1);
        assert_eq!(quarter(&dt(2026, 8, 1, 0, 0, 0)), 3);
    }

    #[test]
    fn formats() {
        let t = dt(2026, 11, 18, 9, 5, 3);
        assert_eq!(format_date(&t), "2026-11-18");
        assert_eq!(format_time(&t), "09:05:03");
        assert_eq!(format_iso(&t), "2026-11-18T09:05:03");
    }
}
