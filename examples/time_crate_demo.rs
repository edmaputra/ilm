use std::time::{SystemTime, UNIX_EPOCH};
use time::{OffsetDateTime, format_description};

fn main() {
    // Get current epoch timestamp in milliseconds (i64)
    let now_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    
    println!("Current timestamp (i64): {}", now_millis);
    
    // Convert i64 milliseconds to OffsetDateTime
    let datetime = OffsetDateTime::from_unix_timestamp_nanos((now_millis * 1_000_000) as i128)
        .unwrap();
    
    // Custom format: dd/MM/yyyy HH:MM:SS.mmm UTC
    let format = format_description::parse(
        "[day]/[month]/[year] [hour]:[minute]:[second].[subsecond digits:3] UTC"
    ).unwrap();
    
    let formatted = datetime.format(&format).unwrap();
    println!("Formatted: {}", formatted);
    
    // Alternative formats
    let iso_format = format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z").unwrap();
    println!("ISO format: {}", datetime.format(&iso_format).unwrap());
    
    // Simple date only
    let date_only = format_description::parse("[day]/[month]/[year]").unwrap();
    println!("Date only: {}", datetime.format(&date_only).unwrap());
    
    // Time calculations are much easier
    let future = datetime + time::Duration::days(7);
    println!("One week later: {}", future.format(&format).unwrap());
    
    // Convert back to i64 milliseconds
    let back_to_millis = datetime.unix_timestamp_nanos() / 1_000_000;
    println!("Back to i64: {}", back_to_millis);
}