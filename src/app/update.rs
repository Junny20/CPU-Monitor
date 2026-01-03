//! Update module.
//!
//! This module contains the main update function for the application loop.

use eframe::egui::Context;

use crate::{
    app::{
        app_monitor::AppMonitor,
        process_monitor::ProcessMonitor,
        state::change_system_monitor_app_state,
        system_monitor::SystemMonitor,
        view::{render_ui, request_repaint, try_receive_latest_snapshot},
    },
    snapshots::{
        cpu_snapshot_struct::CpuSnapshot, processes_snapshot_struct::ProcessesSnapshot,
        system_snapshot_struct::SystemSnapshot,
    },
};

// INVARIANTS:
// change_system_monitor_app_state only runs when a cpu_snapshot is actually received.
//

/// Main update function for the application.
///
/// Handles receiving snapshots from channels, updating monitors, and rendering the UI.
/// This function is called in each frame by the egui framework.
///
/// * Parameters
/// `app_monitor` Mutable reference to the app monitor
/// `ctx` The egui context for UI operations
pub fn update(app_monitor: &mut AppMonitor, ctx: &Context) {
    // NOTE: We use the :: syntax because turbofish parses the `<` symbol as a comparison operator otherwise, this is a known issue with if let statements.
    if let Some(system_snapshot) = try_receive_latest_snapshot::<SystemSnapshot>(
        &app_monitor.channels.system_snapshot_receiver,
    ) {
        let system_monitor: SystemMonitor = SystemMonitor::build_from_snapshot(system_snapshot);
        app_monitor.system_monitor = system_monitor;
    }

    let potential_processes_snapshot: Option<ProcessesSnapshot> =
        try_receive_latest_snapshot(&app_monitor.channels.processes_snapshot_receiver);

    if let Some(processes_snapshot) = potential_processes_snapshot {
        let processes_monitor: ProcessMonitor =
            ProcessMonitor::build_from_snapshot(processes_snapshot);
        app_monitor.process_monitor = processes_monitor;
    }

    let potential_cpu_snapshot: Option<CpuSnapshot> =
        try_receive_latest_snapshot(&app_monitor.channels.cpu_snapshot_receiver);

    if let Some(cpu_snapshot) = potential_cpu_snapshot {
        change_system_monitor_app_state(cpu_snapshot, app_monitor);
    };

    // refreshes the gui
    request_repaint(ctx);
    render_ui(ctx, app_monitor);
}
