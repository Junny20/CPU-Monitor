//! Process monitor module.
//!
//! This module defines the ProcessMonitor struct for tracking system processes.

use crate::ProcessesSnapshot;

/// Process monitor structure.
///
/// Stores information about the number of running processes in the system.
pub struct ProcessMonitor {
    pub processes: usize,
}

impl ProcessMonitor {
    /// Builds a ProcessMonitor from a processes snapshot.
    ///
    /// * Parameters
    /// `processes_snapshot_struct` The snapshot containing process data
    ///
    /// * Returns
    /// A ProcessMonitor instance with the process count
    pub fn build_from_snapshot(processes_snapshot_struct: ProcessesSnapshot) -> ProcessMonitor {
        ProcessMonitor {
            processes: processes_snapshot_struct.processes,
        }
    }

    /// Creates a new ProcessMonitor with default values.
    ///
    /// * Returns
    /// A ProcessMonitor instance with processes set to 0
    pub fn new() -> ProcessMonitor {
        ProcessMonitor { processes: 0 }
    }
}
