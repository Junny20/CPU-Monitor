//! Processes snapshot structure.
//!
//! Represents a snapshot of process count data.

/// Structure holding process count information.
/// 
/// Contains the number of running processes.
pub struct ProcessesSnapshot {
    pub processes: usize,
}

impl ProcessesSnapshot {
    /// Creates a new ProcessesSnapshot with default values.
    /// 
    /// * Returns
    /// A ProcessesSnapshot with zero processes
    fn new() -> ProcessesSnapshot {
        ProcessesSnapshot { processes: 0 }
    }
}
