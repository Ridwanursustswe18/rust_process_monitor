# rust_process_monitor
- We can use commmand when using cargo for example like this 'cargo run -- -monitorFile D:\task-rust\assets/monitors.json'
- This command then will be parsed and process_monitor function will be called for five minutes.
- In the process_montior function it will call update_monitors and store_monitors function in threads to execute them concurrently
- In the update_monitors function result will be updated every 30 seconds
- In the store_monitors function every minute a new json file will be create with format epoch_timestamp_monitors.json
- after execution it will genreate a output.json file which will contain the new structure of the json that was required in the assignment