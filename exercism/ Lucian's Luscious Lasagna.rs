// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let total_time: i32 = 40;
    let remaining_minutes = expected_minutes_in_oven() - actual_minutes_in_oven;
    return remaining_minutes;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let time_for_each_layer: i32 = 2;
    let time_for_layers = (number_of_layers * time_for_each_layer);
    return time_for_layers;
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
}
