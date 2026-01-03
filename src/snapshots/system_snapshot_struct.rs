//! System snapshot structure.
//!
//! Represents a snapshot of system information data.

/// Structure holding system information.
/// 
/// Contains system name, version, architecture, and host name.
pub struct SystemSnapshot {
    pub system_name: String,
    pub system_version: String,
    pub system_architecture: String,
    pub host_name: String,
}

impl SystemSnapshot {
    /// Creates a new SystemSnapshot with default placeholder values.
    /// 
    /// * Returns
    /// A SystemSnapshot with "N/A" for all fields
    pub fn new() -> SystemSnapshot {
        SystemSnapshot {
            system_name: String::from("N/A"),
            system_version: String::from("N/A"),
            system_architecture: String::from("N/A"),
            host_name: String::from("N/A"),
        }
    }
}
