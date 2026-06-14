use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_raw_data() -> io::Result<(String, String, String)> {
    let file = File::open("/proc/cpuinfo")?;
    let reader = BufReader::new(file);
    
    let mut model: String = String::new();
    let mut threads: String = String::new();
    let mut cores: String = String::new(); 
    let mut is_done: [bool;3] = [false,false,false];

    for line_result in reader.lines() {
        let line = line_result?;
        let clean_line = line.trim();

        if clean_line.starts_with("model name") {
            if let Some((_,raw_model)) = clean_line.split_once(":") {
                model = raw_model.trim().to_string();
                is_done[0] = true; 
            }
        }

        if clean_line.starts_with("siblings") {
            if let Some((_,raw_threads)) = clean_line.split_once(":") {
                threads = raw_threads.trim().to_string();
                is_done[1] = true;
            }
        }

        if clean_line.starts_with("cpu cores") {
            if let Some((_,raw_cores)) = clean_line.split_once(":") {
                cores = raw_cores.trim().to_string();
                is_done[2] = true;
            }
        }

        if is_done[0] && is_done[1] && is_done[2] {
            break;
        } 
    }

    return Ok((model, threads, cores));
}