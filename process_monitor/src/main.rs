use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize,Clone, Serialize)]
struct Monitor {
    monitor_id: Option<i32>,
    name: String,
    script: Option<String>,
    result: Option<MonitorResult>,
    #[serde(rename = "type")]
    monitor_type: Option<String>,
    code: String,
}

#[derive(Debug, Deserialize,Clone, Serialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize,Clone, Serialize)]
struct MonitorResult {
    value: i32,
    processed_at: i32,
}

fn parse_arguments() -> Result<Monitors, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Insufficient arguments provided. Usage: -monitorFile <file_path>".to_string());
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
    let  monitors: Monitors;

    match parse_arguments() {
        Ok(parsed_monitors) => {
            monitors = parsed_monitors;
            println!("Number of monitors: {}", monitors.monitors.len());
            let shared_monitors = Arc::new(Mutex::new(monitors.clone())); 
            process_monitors(shared_monitors);
            
           
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
        
    }
}

fn update_monitors(shared_monitors: Arc<Mutex<Monitors>>) {
    let start_time = Instant::now();

    loop {
       
        if start_time.elapsed() >= Duration::from_secs(300) {
            break;
        }

        
        thread::sleep(Duration::from_secs(30));
        let mut monitors = shared_monitors.lock().unwrap();
        for monitor in &mut monitors.monitors {
            let result = generate_result();
            monitor.result = Some(result);
        }
        println!("{:#?}", monitors);
    }
}


fn store_monitors(shared_monitors: Arc<Mutex<Monitors>>) {
    let start_time = Instant::now();

    loop {
        let elapsed_time = start_time.elapsed();
        if elapsed_time >= Duration::from_secs(300) {
            break;
        }

        
        thread::sleep(Duration::from_secs(60));

        let monitors = shared_monitors.lock().unwrap();
        let json_string = serde_json::to_string_pretty(&*monitors).unwrap();
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let file_name = format!("{}_monitors.json", current_timestamp);
        fs::write(&file_name, json_string).expect("Unable to write file");
    }
}


fn process_monitors(shared_monitors: Arc<Mutex<Monitors>>) {
   

  

        
        let update_thread = thread::spawn({
            let shared_monitors = Arc::clone(&shared_monitors);
            move || {
                update_monitors(shared_monitors);
            }
        });

        let store_thread = thread::spawn({
            let shared_monitors = Arc::clone(&shared_monitors);
            move || {
                store_monitors(shared_monitors);
            }
        });


        update_thread.join().unwrap();
        store_thread.join().unwrap();
    
}


fn generate_result() -> MonitorResult {
    let value = rand::thread_rng().gen_range(1..100);
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let processed_at = since_epoch.as_secs() as i32;
    MonitorResult { value, processed_at }
}
