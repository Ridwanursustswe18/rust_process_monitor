use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct Monitor {
    monitor_id: Option<i32>,
    name: String,
    script: Option<String>,
    result: Option<String>,
    #[serde(rename = "type")]
    monitor_type: Option<String>,
    code: String,
}

#[derive(Debug, Deserialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

fn parse_arguments() -> Result<Monitors, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return Err("Insufficient arguments provided. Usage: process_monitor -monitorFile <file_path>".to_string());
    }

    if args[1] != "process_monitor" {
        return Err("Invalid command. Must start with 'process_monitor'.".to_string());
    }

    if args[2] != "-monitorFile" {
        return Err("Invalid argument. Second argument must be '-monitorFile'.".to_string());
    }

    let file_path = &args[3];
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
    let monitors: Monitors;

    match parse_arguments() {
        Ok(parsed_monitors) => {
            monitors = parsed_monitors;
             println!("Number of monitors: {}", monitors.monitors.len());
             for monitor in &monitors.monitors {
                println!("Monitor ID: {:?}", monitor.monitor_id);
                println!("Name: {}", monitor.name);
                println!("Script: {:?}", monitor.script);
                println!("Result: {:?}", monitor.result);
                println!("Monitor Type: {:?}", monitor.monitor_type);
                println!("Code: {}", monitor.code);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
 
    
}
