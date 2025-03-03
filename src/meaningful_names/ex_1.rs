#[allow(dead_code)]

pub struct BadCode {
    // If name requires a comment, name doesn't reveal its intent
    d: u32, // elapsed time in days

    // Alternatives
    elapsed_time_in_days: u32,
    days_since_creation: u32,
    days_since_modification: u32,
    file_age_in_days: u32,
}
