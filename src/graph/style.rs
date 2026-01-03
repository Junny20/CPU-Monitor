//! Graph styling utilities.
//!
//! Provides functions for determining colors and line thicknesses based on CPU usage values.

use crate::config::style::{
    GREEN_LINE_THICKNESS, RED_LINE_THICKNESS, RED_LINE_THRESHOLD, YELLOW_LINE_THICKNESS,
    YELLOW_LINE_THRESHOLD,
};
use eframe::egui::Color32;

/// Determines the color for a given CPU usage value.
/// 
/// Returns green for low usage, yellow for medium, red for high.
/// 
/// * Parameters
/// `value` The CPU usage percentage
/// `opacity` The alpha value for the color
/// 
/// * Returns
/// The appropriate Color32
pub fn get_color(value: f32, opacity: u8) -> Color32 {
    match value {
        value if value < YELLOW_LINE_THRESHOLD => {
            Color32::from_rgba_unmultiplied(0, 255, 0, opacity)
        }

        value if value < RED_LINE_THRESHOLD => {
            Color32::from_rgba_unmultiplied(255, 255, 0, opacity)
        }
        // red line otherwise
        _ => Color32::from_rgba_unmultiplied(255, 0, 0, opacity),
    }
}

/// Determines the stroke width for a given CPU usage value.
/// 
/// * Parameters
/// `value` The CPU usage percentage
/// 
/// * Returns
/// The appropriate line thickness
pub fn find_stroke_width(value: f32) -> f32 {
    match value {
        value if value < YELLOW_LINE_THRESHOLD => GREEN_LINE_THICKNESS,
        value if value < RED_LINE_THRESHOLD => YELLOW_LINE_THICKNESS,
        _ => RED_LINE_THICKNESS,
    }
}
