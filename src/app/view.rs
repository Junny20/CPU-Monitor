use std::time::Duration;

use crate::{
    app::app::AppMonitor,
    config::{
        app_variables::REFRESH_MILLISECONDS,
        layout::{
            CELL_HEIGHT_PX, LEFT_CELL_WIDTH_PX, PROGRESS_BAR_HEIGHT_PX, PROGRESS_BAR_ROUNDING_PX,
            PROGRESS_BAR_SPACING_PX, PROGRESS_BAR_WIDTH_PX, TEXT_SPACING_PX,
        },
        style::HALF_OPACITY,
    },
    graph::{draw::draw_ui_graph, style::get_color},
    snapshots::{cpu_snapshot_struct::CpuSnapshot, processes_snapshot_struct::ProcessesSnapshot, system_snapshot_struct::SystemSnapshot},
};

use eframe::egui::{
    vec2, Align, CentralPanel, Color32, Context, Layout, ProgressBar, Response, ScrollArea, Sense,
    UiBuilder, Vec2,
};

pub fn try_receive_system_snapshot(app_monitor: &mut AppMonitor) -> Option<SystemSnapshot> {
    if let Ok(system_snapshot) = app_monitor.channels.system_snapshot_receiver.try_recv() {
        Some(system_snapshot)
    } else {
        None
    }
}

pub fn try_receive_latest_cpu_snapshot(app_monitor: &mut AppMonitor) -> Option<CpuSnapshot> {
    let mut latest: Option<CpuSnapshot> = None;

    while let Ok(cpu_snapshot) = app_monitor.channels.cpu_snapshot_receiver.try_recv() {
        latest = Some(cpu_snapshot);
    }

    latest
}

pub fn try_receive_latest_processes_snapshot(app_monitor: &mut AppMonitor) -> Option<ProcessesSnapshot> {
    let mut latest: Option<ProcessesSnapshot> = None;

    while let Ok(processes_snapshot) = app_monitor.channels.processes_snapshot_receiver.try_recv() {
        latest = Some(processes_snapshot);
    }

    latest
} // TODO: MAKE THIS FUNCTION GENERIC!

pub fn render_ui(ctx: &Context, app_monitor: &mut AppMonitor) {
    // the show method takes a closure and builds the gui
    CentralPanel::default().show(ctx, |ui| {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                ui.scope_builder(
                    UiBuilder::new()
                        .max_rect(rect)
                        .layout(Layout::left_to_right(Align::Center)),
                    |ui| {
                        ui.heading("CPU Monitor");
                    },
                );

                let right_cell: Vec2 = vec2(ui.available_width(), CELL_HEIGHT_PX);
                let (rect, _) = ui.allocate_exact_size(right_cell, Sense::hover());
                ui.scope_builder(
                    UiBuilder::new()
                        .max_rect(rect)
                        .layout(Layout::right_to_left(Align::Center)),
                    |ui| {
                        ui.label("Average Usage: ");
                        ui.add_space(50.0);
                        ui.label(format!("Processes: {}", app_monitor.process_monitor.processes));
                        ui.add_space(50.0);
                        ui.label(format!(
                            "OS: {} {} {}",
                            app_monitor.system_monitor.system_name,
                            app_monitor.system_monitor.system_version,
                            app_monitor.system_monitor.system_architecture
                        ));
                        ui.add_space(50.0);
                    },
                )
            })
        });

        // ===== OVERALL CPU USAGE =====
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                    // space to align with CPU Core labels
                    ui.label("CPU ");
                    ui.add_space(TEXT_SPACING_PX);

                    if let Some(overall_cpu_usage) =
                        app_monitor.cpu_monitor.overall_cpu_history.back()
                    {
                        // value formatted to one decimal place
                        ui.monospace(format!("{:>5.1}%", overall_cpu_usage));
                        ui.add_space(PROGRESS_BAR_SPACING_PX); // magic nums

                        let color: Color32 = get_color(*overall_cpu_usage, HALF_OPACITY);

                        let progress_bar: ProgressBar = app_monitor.cpu_monitor.build_progress_bar(
                            *overall_cpu_usage,
                            PROGRESS_BAR_WIDTH_PX,
                            PROGRESS_BAR_HEIGHT_PX,
                            PROGRESS_BAR_ROUNDING_PX,
                            color,
                        );

                        let _response: Response = ui.add(progress_bar);

                        ui.add_space(TEXT_SPACING_PX);
                    }
                });

                let right_cell: Vec2 = vec2(ui.available_width(), CELL_HEIGHT_PX);
                let (rect, _) = ui.allocate_exact_size(right_cell, Sense::hover());

                draw_ui_graph(
                    &rect,
                    ui,
                    &app_monitor.cpu_monitor.overall_cpu_history,
                    Some(&app_monitor.cpu_monitor.overall_ema_cpu_history),
                );
            });
        });

        ui.add_space(TEXT_SPACING_PX);

        ScrollArea::vertical().show(ui, |ui| {
            // ===== PER CORE CPU USAGE =====
            if let Some(per_core_history) = &app_monitor.cpu_monitor.per_core_cpu_history {
                for (index, history) in per_core_history.iter().enumerate() {
                    let usage: &f32 = history.back().unwrap(); // Check if this always works!

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                            let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                                ui.label(format!("Core {}", index));
                                // value formatted to one decimal place
                                ui.monospace(format!("{:>5.1}%", *usage));
                                ui.add_space(PROGRESS_BAR_SPACING_PX);

                                let color: Color32 = get_color(*usage, HALF_OPACITY);

                                let progress_bar: ProgressBar =
                                    app_monitor.cpu_monitor.build_progress_bar(
                                        *usage,
                                        PROGRESS_BAR_WIDTH_PX,
                                        PROGRESS_BAR_HEIGHT_PX,
                                        PROGRESS_BAR_ROUNDING_PX,
                                        color,
                                    );

                                let _response: Response = ui.add(progress_bar);

                                ui.add_space(TEXT_SPACING_PX);
                            });

                            let desired_size = vec2(ui.available_width(), CELL_HEIGHT_PX);
                            let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());

                            draw_ui_graph(
                                &rect,
                                ui,
                                history,
                                Some(&app_monitor.cpu_monitor.overall_ema_cpu_history),
                            );
                        });
                    });

                    ui.add_space(TEXT_SPACING_PX);
                }
            }
        });
    });
}

pub fn request_repaint(ctx: &Context) {
    // refreshes the gui every REFRESH_MILLISECONDS milliseconds
    ctx.request_repaint_after(Duration::from_millis(REFRESH_MILLISECONDS));
}
