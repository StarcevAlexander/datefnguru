# datefn-guru

**date-fns style date utilities for Rust** — 25+ focused helpers over `chrono::NaiveDateTime`: arithmetic, boundaries (`start_of_day`, `end_of_month`...), comparisons, differences and formatting, all pure functions.

Formerly published on crates.io as `datefnguru`.

For developers coming from JavaScript's [date-fns](https://date-fns.org) who miss small composable functions instead of one giant method object.

## Usage

```rust
use chrono::NaiveDateTime;
use datefn_guru::{add_months, days_in_month, end_of_month, format_date, is_weekend, start_of_day};

fn main() {
    let dt = NaiveDateTime::from_timestamp_opt(0, 0).unwrap().with_year(2026).unwrap().with_month(2).unwrap().with_day(10).unwrap();

    assert_eq!(days_in_month(2026, 2), 28);
    assert_eq!(format_date(&start_of_day(&dt)), "2026-02-10");

    let next_quarter_start = add_months(&start_of_day(&dt), 1);
    let _ = (next_quarter_start, is_weekend(&dt), end_of_month(&dt));
}
```

## Functions

`add_days` · `add_hours` · `add_minutes` · `add_seconds` · `add_months` (clamps to last day of month) · `add_years` · `start_of_day` · `end_of_day` · `start_of_week` · `end_of_week` · `start_of_month` · `end_of_month` · `start_of_year` · `end_of_year` · `is_same_day` · `is_same_month` · `is_same_year` · `is_weekend` · `is_leap_year` · `days_in_month` · `difference_in_days` · `difference_in_hours` · `difference_in_minutes` · `difference_in_seconds` · `quarter` · `format_iso` · `format_date` · `format_time`

## License

Licensed under either of MIT or Apache-2.0, at your option.

Name is a tribute to [date-fns](https://date-fns.org); this crate is an independent implementation over chrono for Rust.
