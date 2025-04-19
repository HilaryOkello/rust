use std::collections::HashMap;
use chrono::{DateTime, Datelike, Utc};

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut author_counts = HashMap::new();
    
    for commit in data.members() {
        if let Some(author) = &commit["author"]["login"].as_str() {
            *author_counts.entry(author.to_string()).or_insert(0) += 1;
        }
    }
    
    author_counts
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();
    
    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let date_utc = date.with_timezone(&Utc);
                let iso_week = date_utc.iso_week();
                let year = date_utc.year();
                let week = iso_week.week();
                
                let week_key = format!("{}-W{}", year, week);
                *week_counts.entry(week_key).or_insert(0) += 1;
            }
        }
    }
    
    week_counts
}