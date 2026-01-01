pub struct SystemSnapshot {
    pub system_name: String,
    pub system_version: String,
    pub system_architecture: String,
    pub host_name: String,
}

impl SystemSnapshot {
    pub fn new() -> SystemSnapshot {
        SystemSnapshot {
            system_name: String::from("N/A"),
            system_version: String::from("N/A"),
            system_architecture: String::from("N/A"),
            host_name: String::from("N/A"),
        }
    }
}
