use sysinfo::System;

pub struct SystemInfo {
    sys: System,
}


impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo { sys: System::new_all() }
    }

    pub fn get_cpu_usage_percentage(&mut self) -> f32 {
        self.sys.refresh_cpu();
        let cpu_usage_percent = self.sys.global_cpu_info().cpu_usage() as f32;
        cpu_usage_percent
    }

    pub fn get_ram_usage_percentage(&mut self) -> f32 {
        self.sys.refresh_memory();
        let total_memory = self.sys.total_memory() as f32;
        let used_memory = self.sys.used_memory() as f32;
        let ram_usage_percent = (used_memory / total_memory) * 100.0;
        ram_usage_percent
    }
}