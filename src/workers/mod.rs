pub mod processes;
pub mod send_system_snapshot;

use crate::snapshots::cpu_snapshot_struct::CpuSnapshot;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use sysinfo::System;

pub fn send_cpu_snapshot(sender: Sender<CpuSnapshot>) {
    let mut sys = System::new();

    let _handle: JoinHandle<()> = thread::spawn(move || {
        loop {
            sys.refresh_cpu_usage();
            let overall_cpu_usage: f32 = sys.global_cpu_info().cpu_usage();
            let mut per_core_cpu_usage: Vec<f32> = Vec::new();
            for cpu in sys.cpus() {
                let cpu_usage: f32 = cpu.cpu_usage();
                per_core_cpu_usage.push(cpu_usage);
            }
            let cpu_snapshot: CpuSnapshot = CpuSnapshot {
                overall_cpu_usage: overall_cpu_usage,
                per_core_cpu_usage: per_core_cpu_usage,
            };
            if let Err(e) = sender.send(cpu_snapshot) {
                eprintln!("Error: {}", e);
                return;
            }
            // Sleeping to let time for the system to run for long
            // enough to have useful information.
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}
