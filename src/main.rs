mod app;
mod channel;
mod workers;
mod cpusnapshot;

use app::SystemMonitorApp;
use channel::Channel;
use cpusnapshot::CpuSnapshot;

fn main() -> eframe::Result<()> {
    let channel: Channel<CpuSnapshot> = Channel::new();
    let (sender, receiver) = channel.split();
    workers::cpu(sender);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust System Monitor",
        options,
        Box::new(|_cc| Box::new(SystemMonitorApp::new(receiver))),
    )
}
