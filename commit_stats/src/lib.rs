use chrono::{DateTime, Datelike, Utc, Weekday};
use json::JsonValue;
use std::collections::HashMap;

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut author_counts = HashMap::new();

    for commit in data.members() {
        if let Some(author) = commit["author"]["login"].as_str() {
            *author_counts.entry(author.to_string()).or_insert(0) += 1;
        }
    }

    author_counts
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(datetime) = DateTime::parse_from_rfc3339(date_str) {
                let datetime_utc = datetime.with_timezone(&Utc);
                let iso_week = datetime_utc.iso_week();
                let key = format!("{}-W{}", iso_week.year(), iso_week.week());
                *week_counts.entry(key).or_insert(0) += 1;
            }
        }
    }

    week_counts
}