use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::os::windows::process;
struct Process{ // struuct of the process
    name: String,
    execution_time: u8,
    execution_time_temp: u8,
    arrival_time: u8,
    compeletion_time:u8,
    waiting_time:u8,
    turnaround_time:u8
}
fn main() {
    let filename = "info.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut temp: Vec<String> = Vec::new();
    let mut name: Vec<String> = Vec::new();
    let mut arrival: Vec<u8> = Vec::new();
    let mut execute: Vec<u8> = Vec::new();
    let mut processces: Vec<Process> = Vec::new();
    let mut queue: VecDeque<Process> = VecDeque::new();



    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        lines.push(line)
    }
    for line in lines {
        for words in line.split(" ") { 
            temp.push(words.to_string());
        }
        }
    for i in 0..temp.len()/3 { // split execute time , name and arrival time to different vectors.
        name.push(temp[i * 3].to_string());
        execute.push(temp[i*3+1].parse::<u8>().unwrap());
        arrival.push(temp[i*3+2].parse::<u8>().unwrap());
    }
    for i in 0..name.len() { // put all of them as struct process in a vector.
        let process: Process = Process{name:name[i].to_string(),execution_time:execute[i],arrival_time:arrival[i], compeletion_time:0,waiting_time:0,turnaround_time:0, execution_time_temp:execute[i]};
        processces.push(process);
    }
    processces.sort_by_key(|process: &Process| process.arrival_time); // sort by arrival time.

    let mut timer:u8 = 0; 
    let tq:u8 = 2; // quantum time.
    let mut compeletd_tasks: Vec<Process> = Vec::new();   // the vactor with the process that compeleted.
    let mut temp_queue: Vec<Process> = Vec::new(); // temp vector for the process that is waiting.

    loop {
        // check for new processes and add them to the queue
        while processces.len() > 0 && processces[0].arrival_time <= timer {
            queue.push_back(processces.remove(0));
        }
    
        if queue.len() > 0 {
            let mut process = queue.pop_front().unwrap();
    
            // execute the process for the time quantum
            if process.execution_time > tq {
                process.execution_time -= tq;
                timer += tq;
                temp_queue.push(process);
            } else {
                timer += process.execution_time;
                process.execution_time = 0;
                process.compeletion_time = timer;
                process.turnaround_time = process.compeletion_time - process.arrival_time;
                process.waiting_time = process.turnaround_time - process.execution_time_temp;
                compeletd_tasks.push(process);
            }
        } else if processces.len() == 0 {
            // all processes have been executed
            break;
        } else {
            // no processes in queue, wait for next process
            timer += 1;
        }
    
        // add any new processes that have arrived while the current process was executing
        while processces.len() > 0 && processces[0].arrival_time <= timer {
            queue.push_back(processces.remove(0));
        }
    
        // add any processes that did not complete during the time quantum back to the queue
        while temp_queue.len() > 0 {
            queue.push_back(temp_queue.remove(0));
        }
    }
    let mut waiting : f32 = 0.0;
    let mut turnaround: f32 = 0.0;
    for process in compeletd_tasks.iter()  {

        waiting += process.waiting_time as f32;
        turnaround += process.turnaround_time as f32;
    }

    for process in compeletd_tasks.iter() {
        print!("{}",process.waiting_time);
    }
    let length = compeletd_tasks.len() as f32;
    print!("{}, {}",waiting/length,turnaround/length)
}

