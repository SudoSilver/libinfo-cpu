use std::fs::File;
use std::io::{self, BufRead, BufReader};
use libscrstrings::{ CheckString, ConvertString };

pub fn get_raw_stats() -> io::Result<(u64,u64)> {
    let file = File::open("/proc/stat")?;
    let reader = BufReader::new(file);
    
    let mut raw_cpu_values: Vec<String> = Vec::new();
    let mut cpu_values: Vec<u64> = Vec::new(); 

    for line_result in reader.lines() {
        let line = line_result?;
        
        if line.trim().starts_with("cpu ") {
            raw_cpu_values = line
                .split_whitespace()
                .map(|item| item.to_string())
                .collect();

            break;
        }
    }

    for value in raw_cpu_values {
        if value == "cpu" {
            continue;
        }

        if value.is_uint() {
            // If the next line fails it is an unrecoverable error and something is EXTREMELY wrong
            let num = value.to_uint().expect("[FATAL]: Value is a valid 64 bit unsigned integer but can not be converted to u64 ");
            cpu_values.push(num);
        }
    }

    // In case something breaks the data
    if cpu_values.len() < 5 {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidData, "Malformed /proc/stat structure"));
    }

    let cpu_total = cpu_values.iter().sum(); 

    // column 3 has Idle Ticks and column 4 has IOWait Ticks 
    // their sum is the idle ticks of the cpu
    let cpu_idle = cpu_values[3] + cpu_values[4]; 

    return Ok((cpu_total, cpu_idle));
}