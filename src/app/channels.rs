//! Defines Channels structure and implementation.

use crate::app::app_receivers::AppReceivers;
use crate::{CpuSnapshot, ProcessesSnapshot, SystemSnapshot};
use std::sync::mpsc::Receiver;

/// A sub-structure of the AppMonitor structure.
/// Stores all receivers used in the GUI.
pub struct Channels {
    pub cpu_snapshot_receiver: Receiver<CpuSnapshot>,
    pub system_snapshot_receiver: Receiver<SystemSnapshot>,
    pub processes_snapshot_receiver: Receiver<ProcessesSnapshot>,
}

impl Channels {
    /// Constructor for the Channels structure, consumes an AppReceivers structure.
    ///
    /// # Parameters
    /// * `app_receivers` AppReceivers structure
    ///
    /// # Returns
    /// Channels structure
    pub fn new(app_receivers: AppReceivers) -> Channels {
        Channels {
            cpu_snapshot_receiver: app_receivers.cpu_snapshot_receiver,
            system_snapshot_receiver: app_receivers.system_snapshot_receiver,
            processes_snapshot_receiver: app_receivers.processes_snapshot_receiver,
        }
    }
}
