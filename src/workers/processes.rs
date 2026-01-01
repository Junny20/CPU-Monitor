use crate::snapshots::processes_snapshot_struct::ProcessesSnapshot;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use sysinfo::System;

pub fn send_processes_snapshot(process_sender: Sender<ProcessesSnapshot>) {
    let mut sys: System = System::new_all();
    let _handle: JoinHandle<()> = thread::spawn(move || loop {
        sys.refresh_processes();
        let process_count: usize = System::processes(&sys).len();

        if let Err(e) = process_sender.send(ProcessesSnapshot {
            processes: process_count,
        }) {
            eprintln!("Error sending process count: {}", e);
            return;
        }

        thread::sleep(Duration::from_millis(500));
    });
}
