//! State management module.
//!
//! This module contains functions for updating the application state based on received snapshots.

use crate::{app::app_monitor::AppMonitor, snapshots::cpu_snapshot_struct::CpuSnapshot};

// INVARIANTS:
// This function only triggers when a valid cpu_snapshot is received.
// As such, cpu_snapshot is not an Option type - it is guaranteed to exist.

/// Changes the system monitor app state by applying a CPU snapshot.
///
/// Updates the CPU monitor within the app monitor with the new snapshot data.
///
/// * Parameters
/// `cpu_snapshot` The CPU snapshot to apply
/// `app_monitor` Mutable reference to the app monitor to update
pub fn change_system_monitor_app_state(cpu_snapshot: CpuSnapshot, app_monitor: &mut AppMonitor) {
    app_monitor
        .cpu_monitor
        .cpu_monitor_apply_cpu_snapshot(cpu_snapshot);
}
