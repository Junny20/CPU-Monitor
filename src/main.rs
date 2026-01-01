mod app;
mod channel;
mod config;
mod data;
mod graph;
mod snapshots;
mod workers;

use app::app::AppMonitor;
use channel::Channel;
use snapshots::cpu_snapshot_struct::CpuSnapshot;
use workers::{send_cpu_snapshot, send_system_snapshot::send_system_snapshot};

use crate::{
    app::app::AppReceivers, config::layout::{APP_HEIGHT_PX, APP_WIDTH_PX}, snapshots::{
        processes_snapshot_struct::ProcessesSnapshot, system_snapshot_struct::SystemSnapshot,
    }, workers::processes::send_processes_snapshot
};

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
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([APP_WIDTH_PX, APP_HEIGHT_PX]),
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
