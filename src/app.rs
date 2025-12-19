use egui::*;
use std::sync::mpsc::Receiver;
use crate::cpusnapshot::CpuSnapshot;

pub struct SystemMonitorApp {
    receiver: Receiver<CpuSnapshot>,
    latest_snapshot: Option<CpuSnapshot>
}

impl SystemMonitorApp {
    pub fn new(receiver: Receiver<CpuSnapshot>) -> Self {
        Self {
            receiver,
            latest_snapshot: None,
        }
    }
}

impl eframe::App for SystemMonitorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Ok(cpu_snapshot) = self.receiver.try_recv() {
            self.latest_snapshot = Some(cpu_snapshot);
            ctx.request_repaint();
            // todo!("Make request_repaint() more efficient");
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust System Monitor");
            ui.add_space(10.0);

            if let Some(cpu_snapshot) = &self.latest_snapshot {
                ui.group(|ui| {
                    ui.label("Overall CPU Usage");

                    let usage = cpu_snapshot.overall_cpu_usage / 100.0;
                    ui.add(
                        ProgressBar::new(usage)
                            .text(format!("{:.1}%", cpu_snapshot.overall_cpu_usage))
                    );
                });

                ui.add_space(12.0);

                ui.group(|ui| {
                    ui.label("Per-Core Usage");
                    ui.add_space(6.0);

                    for (index, value) in cpu_snapshot.per_core_cpu_usage.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Core {:>2}", index));

                            ui.add(
                                ProgressBar::new(*value / 100.0)
                                    .desired_width(160.0)
                            );

                            ui.label(format!("{:>5.1}%", value));
                        });
                    }
                });
            } else {
                ui.label("Waiting for CPU data…");
            }
        });
    }
}