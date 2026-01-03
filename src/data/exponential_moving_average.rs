//! Exponential moving average calculations.
//!
//! Provides functions to compute exponential moving averages for CPU usage data.

const SMOOTHING_FACTOR: f32 = 0.4;

/// Calculates the exponential moving average for overall CPU usage.
/// 
/// * Parameters
/// `previous_ema` The previous EMA value, if any
/// `usage` The current CPU usage value
/// 
/// * Returns
/// The new exponential moving average
pub fn get_cpu_exponential_moving_average(previous_ema: Option<f32>, usage: f32) -> f32 {
    match previous_ema {
        Some(previous_ema) => calculate_exponential_moving_average(previous_ema, usage),
        None => usage,
    }
}

/// Calculates exponential moving averages for per-core CPU usage.
/// 
/// Updates the provided vector of previous EMAs in place.
/// 
/// * Parameters
/// `previous_emas` Mutable reference to vector of previous EMA values
/// `usage` Reference to vector of current per-core usage values
pub fn get_per_core_exponential_moving_average(
    previous_emas: &mut Vec<Option<f32>>,
    usage: &Vec<f32>,
) {
    for (index, ema) in previous_emas.iter_mut().enumerate() {
        *ema = match *ema {
            Some(ema) => Some(calculate_exponential_moving_average(ema, usage[index])),
            None => Some(usage[index]),
        }
    }
}

/// Computes the exponential moving average formula.
/// 
/// * Parameters
/// `previous_ema` The previous EMA value
/// `usage` The current usage value
/// 
/// * Returns
/// The calculated EMA
pub fn calculate_exponential_moving_average(previous_ema: f32, usage: f32) -> f32 {
    usage * SMOOTHING_FACTOR + previous_ema * (1 as f32 - SMOOTHING_FACTOR)
}
