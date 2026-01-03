use crate::app::{
    app_receivers::AppReceivers, channels::Channels, cpu_monitor::CpuMonitor,
    process_monitor::ProcessMonitor, system_monitor::SystemMonitor, update::update,
};

use eframe::egui::Context;

/// Main application monitor structure.
///
/// This struct represents the core of the CPU monitoring application.
/// It holds all the sub-monitors and channels for receiving data snapshots.
/// Implements the eframe::App trait to integrate with the egui framework.
pub struct AppMonitor {
    pub channels: Channels,
    pub cpu_monitor: CpuMonitor,
    pub system_monitor: SystemMonitor,
    pub process_monitor: ProcessMonitor,
}

impl AppMonitor {
    /// Creates a new AppMonitor instance.
    ///
    /// * Parameters
    /// `app_receivers` The receivers for various data snapshots
    ///
    /// * Returns
    /// A new AppMonitor instance
    pub fn new(app_receivers: AppReceivers) -> Self {
        Self {
            channels: Channels::new(app_receivers),
            cpu_monitor: CpuMonitor::new(),
            system_monitor: SystemMonitor::new(),
            process_monitor: ProcessMonitor::new(),
        }
    }
}

impl eframe::App for AppMonitor {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        update(self, ctx);
    }
}
