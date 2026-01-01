pub struct CpuSnapshot {
    pub overall_cpu_usage: f32,
    pub per_core_cpu_usage: Vec<f32>,
}

impl CpuSnapshot {
    fn new() -> CpuSnapshot {
        CpuSnapshot {
            overall_cpu_usage: 0.0,
            per_core_cpu_usage: Vec::new(),
        }
    }
}
