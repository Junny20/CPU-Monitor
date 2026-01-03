//! System snapshot worker.
//!
//! Background worker for collecting system information data.

use crate::snapshots::system_snapshot_struct::SystemSnapshot;
use std::sync::mpsc::Sender;
use std::thread;
use sysinfo::System;

/// Starts a background worker to collect system information snapshot.
/// 
/// Spawns a thread that collects system data and sends a SystemSnapshot via the channel.
/// 
/// * Parameters
/// `sender` The channel sender for SystemSnapshot data
pub fn send_system_snapshot(sender: Sender<SystemSnapshot>) {
    thread::spawn(move || {
        let mut system_snapshot: SystemSnapshot = SystemSnapshot::new();

        if let Some(system_name) = System::name() {
            system_snapshot.system_name = system_name;
        }

        if let Some(system_version) = System::os_version() {
            system_snapshot.system_version = system_version;
        }

        if let Some(system_architecture) = System::cpu_arch() {
            system_snapshot.system_architecture = system_architecture;
        }

        if let Some(host_name) = System::host_name() {
            system_snapshot.host_name = host_name;
        }

        if let Err(e) = sender.send(system_snapshot) {
            eprintln!("Error: {}", e);
            return;
        }
    });
}
