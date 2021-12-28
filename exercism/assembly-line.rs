// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const RATE_FACTOR: f64 = 221.00;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let ideal_rate: f64 = (speed as f64) * RATE_FACTOR;
    match speed {
        1..=4 => ideal_rate,
        5..=8 => 0.9 * ideal_rate,
        9..=10 => 0.77 * ideal_rate,
        _ => ideal_rate,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let mins_in_hour: u32 = 60;
    return production_rate_per_hour(speed) as u32 / mins_in_hour;
}
