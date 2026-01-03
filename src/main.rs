//! Main entry point for the CPU Monitor application.
//!
//! This module initializes the application, sets up communication channels,
//! starts background workers for data collection, and launches the GUI.

mod app;
mod channel;
mod config;
mod data;
mod graph;
mod snapshots;
mod workers;

use app::app_monitor::AppMonitor;
use channel::Channel;
use snapshots::cpu_snapshot_struct::CpuSnapshot;
use workers::{send_cpu_snapshot, send_system_snapshot::send_system_snapshot};

use crate::{
    app::app_receivers::AppReceivers,
    config::layout::{APP_HEIGHT_PX, APP_WIDTH_PX},
    snapshots::{
        processes_snapshot_struct::ProcessesSnapshot, system_snapshot_struct::SystemSnapshot,
    },
    workers::processes::send_processes_snapshot,
};

/// Main function.
/// 
/// Initializes communication channels for CPU, system, and processes data,
/// starts background workers to collect snapshots, and runs the eframe GUI application.
/// 
/// * Returns
/// Result indicating success or failure of the application
fn main() -> eframe::Result<()> {
    let cpu_snapshot_channel: Channel<CpuSnapshot> = Channel::new();
    let (cpu_sender, cpu_receiver) = cpu_snapshot_channel.split();
    send_cpu_snapshot(cpu_sender);

    let system_snapshot_channel: Channel<SystemSnapshot> = Channel::new();
    let (system_sender, system_receiver) = system_snapshot_channel.split();
    send_system_snapshot(system_sender);

    let processes_snapshot_channel: Channel<ProcessesSnapshot> = Channel::new();
    let (processes_sender, processes_receiver) = processes_snapshot_channel.split();
    send_processes_snapshot(processes_sender);

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([APP_WIDTH_PX, APP_HEIGHT_PX]),
        ..Default::default()
    };

    let app_receivers: AppReceivers =
        AppReceivers::build(cpu_receiver, system_receiver, processes_receiver);

    eframe::run_native(
        "CPU Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(AppMonitor::new(app_receivers)))),
    )
}
