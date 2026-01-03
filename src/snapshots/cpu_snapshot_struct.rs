//! CPU snapshot structure.
//!
//! Represents a snapshot of CPU usage data at a point in time.

/// Structure holding CPU usage information.
/// 
/// Contains overall CPU usage and per-core usage percentages.
pub struct CpuSnapshot {
    pub overall_cpu_usage: f32,
    pub per_core_cpu_usage: Vec<f32>,
}

impl CpuSnapshot {
    /// Creates a new CpuSnapshot with default values.
    /// 
    /// * Returns
    /// A CpuSnapshot with zero usage and empty per-core vector
    fn new() -> CpuSnapshot {
        CpuSnapshot {
            overall_cpu_usage: 0.0,
            per_core_cpu_usage: Vec::new(),
        }
    }
}
