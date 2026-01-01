use crate::{
    app::{self, update::update},
    config::{
        app_variables::{CORES_UPPER_LIMIT, MAX_LINE_GRAPH_POINTS},
        layout::{
            CELL_HEIGHT_PX, LEFT_CELL_WIDTH_PX, PROGRESS_BAR_HEIGHT_PX, PROGRESS_BAR_ROUNDING_PX,
            PROGRESS_BAR_SPACING_PX, PROGRESS_BAR_WIDTH_PX, TEXT_SPACING_PX,
        },
        style::HALF_OPACITY,
    },
    data::exponential_moving_average::{
        get_cpu_exponential_moving_average, get_per_core_exponential_moving_average,
    },
    graph::{draw::draw_ui_graph, style::get_color},
    snapshots::{
        cpu_snapshot_struct::CpuSnapshot,
        processes_snapshot_struct::ProcessesSnapshot,
        system_snapshot_struct::{self, SystemSnapshot},
    },
};

use eframe::egui::{
    vec2, Align, CentralPanel, Color32, Context, Layout, ProgressBar, Response, ScrollArea, Sense,
    UiBuilder, Vec2,
};

use std::{collections::VecDeque, sync::mpsc::Receiver};

pub struct Channels {
    pub cpu_snapshot_receiver: Receiver<CpuSnapshot>,
    pub system_snapshot_receiver: Receiver<SystemSnapshot>,
    pub processes_snapshot_receiver: Receiver<ProcessesSnapshot>,
}

impl Channels {
    pub fn new(app_receivers: AppReceivers) -> Channels {
        Channels {
            cpu_snapshot_receiver: app_receivers.cpu_snapshot_receiver,
            system_snapshot_receiver: app_receivers.system_snapshot_receiver,
            processes_snapshot_receiver: app_receivers.processes_snapshot_receiver
        }
    }
}

pub struct CpuMonitor {
    pub per_core_cpu_history: Option<Vec<VecDeque<f32>>>,
    pub per_core_ema_cpu_history: Option<Vec<VecDeque<f32>>>,
    pub per_core_previous_ema: Vec<Option<f32>>,
    pub previous_ema: Option<f32>,
    pub overall_cpu_history: VecDeque<f32>,
    pub overall_ema_cpu_history: VecDeque<f32>,
}

impl CpuMonitor {
    pub fn new() -> CpuMonitor {
        CpuMonitor {
            per_core_cpu_history: None,
            per_core_ema_cpu_history: None,
            per_core_previous_ema: Vec::with_capacity(CORES_UPPER_LIMIT),
            previous_ema: None,
            // uses with_capacity instead of new constructor to reduce heap reallocations.
            overall_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
            overall_ema_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
        }
    }

    // INVARIANTS:
    // exponential moving average is guaranteed to exist after the first cpu snapshot.
    // history charts have a maximum of 10 data points - that is what MAX_LINE_GRAPH_POINTS refers to.

    pub fn cpu_monitor_apply_cpu_snapshot(&mut self, cpu_snapshot: CpuSnapshot) {
        self.overall_cpu_history
            .push_back(cpu_snapshot.overall_cpu_usage);
        if self.overall_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            self.overall_cpu_history.pop_front();
        }

        let overall_cpu_exponential_moving_average: f32 =
            get_cpu_exponential_moving_average(self.previous_ema, cpu_snapshot.overall_cpu_usage);

        self.previous_ema = Some(overall_cpu_exponential_moving_average);
        self.overall_ema_cpu_history
            .push_back(overall_cpu_exponential_moving_average);
        if self.overall_ema_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            self.overall_ema_cpu_history.pop_front();
        } // separate function in CpuMonitor struct

        // constructs per core cpu histories if not constructed
        if let None = self.per_core_cpu_history {
            let n: usize = cpu_snapshot.per_core_cpu_usage.len();

            self.per_core_cpu_history = Some(vec![VecDeque::new(); n]);
            self.per_core_ema_cpu_history = Some(vec![VecDeque::new(); n]);

            // constructs per core ema history
            self.per_core_previous_ema = vec![None; n];
        }

        get_per_core_exponential_moving_average(
            &mut self.per_core_previous_ema,
            &cpu_snapshot.per_core_cpu_usage,
        );

        for (index, ema) in self.per_core_previous_ema.iter().enumerate() {
            let per_core_ema_values: &mut VecDeque<f32> =
                &mut self.per_core_ema_cpu_history.as_mut().unwrap()[index];
            per_core_ema_values.push_back(ema.unwrap());
            if per_core_ema_values.len() > MAX_LINE_GRAPH_POINTS {
                per_core_ema_values.pop_front();
            }
        }

        let per_core_cpu_history: &mut Vec<VecDeque<f32>> =
            self.per_core_cpu_history.as_mut().unwrap();

        for (index, value) in cpu_snapshot.per_core_cpu_usage.iter().enumerate() {
            let per_core_values: &mut VecDeque<f32> = &mut per_core_cpu_history[index];
            per_core_values.push_back(*value);
            if per_core_values.len() > MAX_LINE_GRAPH_POINTS {
                per_core_values.pop_front();
            }
        }
    }

    pub fn build_progress_bar(
        &self,
        value: f32,
        width: f32,
        height: f32,
        rounding: f32,
        color: Color32,
    ) -> ProgressBar {
        ProgressBar::new(value / 100 as f32)
            .desired_width(width)
            .desired_height(height)
            .fill(color)
            .corner_radius(rounding)
            .show_percentage()
    }
}

pub struct SystemMonitor {
    pub system_name: String,
    pub system_version: String,
    pub system_architecture: String,
    pub host_name: String,
}

impl SystemMonitor {
    pub fn build_from_snapshot(system_snapshot_struct: SystemSnapshot) -> SystemMonitor {
        SystemMonitor {
            system_name: system_snapshot_struct.system_name,
            system_version: system_snapshot_struct.system_version,
            system_architecture: system_snapshot_struct.system_architecture,
            host_name: system_snapshot_struct.host_name,
        }
    }

    pub fn new() -> SystemMonitor {
        SystemMonitor {
            system_name: String::from("N/A"),
            system_version: String::from("N/A"),
            system_architecture: String::from("N/A"),
            host_name: String::from("N/A"),
        }
    }
}

pub struct ProcessMonitor {
    pub processes: usize,
}

impl ProcessMonitor {
    pub fn build_from_snapshot(processes_snapshot_struct: ProcessesSnapshot) -> ProcessMonitor {
        ProcessMonitor { processes: processes_snapshot_struct.processes }
    }

    fn new() -> ProcessMonitor {
        ProcessMonitor { processes: 0 }
    }
}

pub struct AppReceivers {
    pub cpu_snapshot_receiver: Receiver<CpuSnapshot>,
    pub system_snapshot_receiver: Receiver<SystemSnapshot>,
    pub processes_snapshot_receiver: Receiver<ProcessesSnapshot>,
}

// todo: this should return a result, refactor...
impl AppReceivers {
    pub fn build(
        cpu_snapshot_receiver: Receiver<CpuSnapshot>,
        system_snapshot_receiver: Receiver<SystemSnapshot>,
        processes_snapshot_receiver: Receiver<ProcessesSnapshot>,
    ) -> AppReceivers {
        AppReceivers {
            cpu_snapshot_receiver,
            system_snapshot_receiver,
            processes_snapshot_receiver,
        }
    }
}

pub struct AppMonitor {
    pub channels: Channels,
    pub cpu_monitor: CpuMonitor,
    pub system_monitor: SystemMonitor,
    pub process_monitor: ProcessMonitor,
}

impl AppMonitor {
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
