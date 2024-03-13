use serde::{Deserialize,Serialize};
use std::env;
use std::fs;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize,Serialize)]
struct Monitor {
    monitor_id: Option<i32>,
    name: String,
    script: Option<String>,
    result: Option<MonitorResult>,
    #[serde(rename = "type")]
    monitor_type: Option<String>,
    code: String,
    
}

#[derive(Debug, Deserialize,Serialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}
#[derive(Debug, Deserialize,Serialize)]
struct MonitorResult{
    value:i32,
    processed_at:i32
}
fn parse_arguments() -> Result<Monitors, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Insufficient arguments provided. Usage: process_monitor -monitorFile <file_path>".to_string());
    }


    if args[1] != "-monitorFile" {
        return Err("Invalid argument. Second argument must be '-monitorFile'.".to_string());
    }

    let file_path = &args[2];
    if !file_path.ends_with(".json") {
        return Err("Invalid file format. Must be a JSON file.".to_string());
    }

    let file_content = fs::read_to_string(file_path)
        .map_err(|err| format!("Error reading file: {}", err))?;

  
    let monitors: Monitors = serde_json::from_str(&file_content)
        .map_err(|err| format!("Error parsing JSON: {}", err))?;

    Ok(monitors)
}

fn main() {
    let mut monitors: Monitors;

    match parse_arguments() {
        Ok(parsed_monitors) => {
            monitors = parsed_monitors;
            println!("Number of monitors: {}", monitors.monitors.len());

            for monitor in &mut monitors.monitors {
                let result = generate_result(); 
                monitor.result = Some(result); 
            }
            
            let json_string = serde_json::to_string_pretty(&monitors).unwrap();
            std::fs::write("output.json", json_string).expect("Unable to write file");
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}
fn generate_result() -> MonitorResult {
    let value = rand::thread_rng().gen_range(1..100);
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let processed_at = since_epoch.as_secs() as i32; 
    MonitorResult { value, processed_at }
}