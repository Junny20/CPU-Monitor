//! Application module.
//!
//! This module contains the core structures and logic for the CPU monitoring application.
//! It includes components for monitoring CPU usage, system information, processes, and the GUI.

pub mod app_monitor;
pub mod app_receivers;
mod channels;
mod cpu_monitor;
mod process_monitor;
mod state;
mod system_monitor;
mod update;
mod view;
