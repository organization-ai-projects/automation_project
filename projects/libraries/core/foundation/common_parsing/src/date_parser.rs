// projects/libraries/common_parsing/src/date_parser.rs
/// Parses a date string in the format `YYYY-MM-DD`. Returns the input if valid.
/// (Lightweight validation; does not validate month/day combinations like Feb 30.)
pub fn parse_date(date_str: &str) -> Option<String> {
    if date_str.len() != 10 {
        return None;
    }
    if date_str.as_bytes().get(4) != Some(&b'-') || date_str.as_bytes().get(7) != Some(&b'-') {
        return None;
    }

    let year = &date_str[0..4];
    let month = &date_str[5..7];
    let day = &date_str[8..10];

    let y_ok = year.parse::<u32>().is_ok();
    let m_ok = month
        .parse::<u32>()
        .ok()
        .is_some_and(|m| (1..=12).contains(&m));
    let d_ok = day
        .parse::<u32>()
        .ok()
        .is_some_and(|d| (1..=31).contains(&d));

    if y_ok && m_ok && d_ok {
        Some(date_str.to_string())
    } else {
        None
    }
}
