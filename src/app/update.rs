use eframe::egui::Context;

use crate::{
    app::{
        app::{AppMonitor, ProcessMonitor, SystemMonitor},
        state::change_system_monitor_app_state,
        view::{
            render_ui, request_repaint, try_receive_latest_cpu_snapshot, try_receive_latest_processes_snapshot, try_receive_system_snapshot
        },
    },
    snapshots::{cpu_snapshot_struct::CpuSnapshot, processes_snapshot_struct::ProcessesSnapshot, system_snapshot_struct},
};

// INVARIANTS:
// change_system_monitor_app_state only runs when a cpu_snapshot is actually received.
//

pub fn update(app_monitor: &mut AppMonitor, ctx: &Context) {
    if let Some(system_snapshot) = try_receive_system_snapshot(app_monitor) {
        let system_monitor: SystemMonitor = SystemMonitor::build_from_snapshot(system_snapshot);
        app_monitor.system_monitor = system_monitor;
    }

    let potential_processes_snapshot: Option<ProcessesSnapshot> = try_receive_latest_processes_snapshot(app_monitor);

    if let Some(processes_snapshot) = potential_processes_snapshot {
        let processes_monitor: ProcessMonitor = ProcessMonitor::build_from_snapshot(processes_snapshot);
        app_monitor.process_monitor = processes_monitor;
    }

    let potential_cpu_snapshot: Option<CpuSnapshot> = try_receive_latest_cpu_snapshot(app_monitor);

    if let Some(cpu_snapshot) = potential_cpu_snapshot {
        change_system_monitor_app_state(cpu_snapshot, app_monitor);
    };

    // refreshes the gui
    request_repaint(ctx);
    render_ui(ctx, app_monitor);
}
