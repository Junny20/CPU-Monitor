//! System monitor module.
//!
//! This module defines the SystemMonitor struct for tracking system information.

use crate::SystemSnapshot;

/// System monitor structure.
///
/// Stores system-related information such as name, version, architecture, and host name.
pub struct SystemMonitor {
    pub system_name: String,
    pub system_version: String,
    pub system_architecture: String,
    pub host_name: String,
}

impl SystemMonitor {
    /// Builds a SystemMonitor from a system snapshot.
    ///
    /// * Parameters
    /// `system_snapshot_struct` The snapshot containing system data
    ///
    /// * Returns
    /// A SystemMonitor instance with the system information
    pub fn build_from_snapshot(system_snapshot_struct: SystemSnapshot) -> SystemMonitor {
        SystemMonitor {
            system_name: system_snapshot_struct.system_name,
            system_version: system_snapshot_struct.system_version,
            system_architecture: system_snapshot_struct.system_architecture,
            host_name: system_snapshot_struct.host_name,
        }
    }

    /// Creates a new SystemMonitor with default placeholder values.
    ///
    /// * Returns
    /// A SystemMonitor instance with "N/A" for all fields
    pub fn new() -> SystemMonitor {
        SystemMonitor {
            system_name: String::from("N/A"),
            system_version: String::from("N/A"),
            system_architecture: String::from("N/A"),
            host_name: String::from("N/A"),
        }
    }
}
