mod cpu_stats;
mod cpu_data;

use crate::cpu_stats::get_raw_stats;
use crate::cpu_data::get_raw_data;
use std::io;

pub struct CpuInfo {
    cpu_per: Option<u8>,
    cpu_flat: Option<u64>,
    baseline_total: u64,
    baseline_idle: u64,

    cpu_model: String,
    thread_count: String,
    core_count: String,
}

impl CpuInfo {
    pub fn new() -> io::Result<Self> {
        let (total, idle) = get_raw_stats()?;
        let (model, threads, cores) = get_raw_data()?;

        return Ok(Self {
            cpu_per: None,
            cpu_flat: None,
            baseline_total: total,
            baseline_idle: idle,

            cpu_model: model,
            thread_count: threads,
            core_count: cores,
        });
    }
    pub fn update(&mut self) -> io::Result<bool> {
        let (new_total, new_idle) = get_raw_stats()?;

        let cpu_total_delta = new_total - self.baseline_total;
        let cpu_idle_delta = new_idle - self.baseline_idle;
        
        if cpu_total_delta == 0 { 
            return Ok(false); 
        }

        let used = cpu_total_delta - cpu_idle_delta;
        let used_per = (used * 100) / cpu_total_delta;

        self.cpu_per = Some(used_per as u8);
        self.cpu_flat = Some(used);

        self.baseline_total = new_total;
        self.baseline_idle = new_idle;
        
        return Ok(true);
    }

    pub fn get_cpu_info(&self) -> [&str;3] {
        return [
            &self.cpu_model,
            &self.core_count,
            &self.thread_count,
        ];
    }

    pub fn get_cpu_usage(&self) -> Option<(u8, u64)> {
        let per: u8;
        let total: u64;

        if let Some(used_per) = self.cpu_per {
            per = used_per;
        }else{
            return None;
        }

        if let Some(used) = self.cpu_flat {
            total = used;
        }else{
            return None;
        }
        return Some((per, total));
    }
}