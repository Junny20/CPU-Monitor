//! Defines AppReceivers struct and implementation.
//!
//! The AppReceivers struct is what is used to construct AppMonitor, as it
//! solely builds the Channels sub-structure.

use crate::snapshots::{
    cpu_snapshot_struct::CpuSnapshot, processes_snapshot_struct::ProcessesSnapshot,
    system_snapshot_struct::SystemSnapshot,
};

use std::sync::mpsc::Receiver;

/// AppReceivers struct.
/// Stores all receivers made by the .split() function when used on a mpsc.
pub struct AppReceivers {
    pub cpu_snapshot_receiver: Receiver<CpuSnapshot>,
    pub system_snapshot_receiver: Receiver<SystemSnapshot>,
    pub processes_snapshot_receiver: Receiver<ProcessesSnapshot>,
}

impl AppReceivers {
    /// Constructs an AppReceivers struct from given Receiver<T>s.
    ///
    /// * Parameters
    /// `cpu_snapshot_receiver` Receiver for CpuSnapshot structure.
    pub fn build(
        cpu_snapshot_receiver: Receiver<CpuSnapshot>,
        system_snapshot_receiver: Receiver<SystemSnapshot>,
        processes_snapshot_receiver: Receiver<ProcessesSnapshot>,
    ) -> AppReceivers {
        AppReceivers {
            cpu_snapshot_receiver,
            system_snapshot_receiver,
            processes_snapshot_receiver,
        }
    }
}
