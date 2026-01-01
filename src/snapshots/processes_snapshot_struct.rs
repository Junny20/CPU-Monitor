pub struct ProcessesSnapshot {
    pub processes: usize,
}

impl ProcessesSnapshot {
    fn new() -> ProcessesSnapshot {
        ProcessesSnapshot { processes: 0 }
    }
}
