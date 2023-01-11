use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::Read;
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent}; 
use nix::unistd::{fork, getpid, getppid, execv};
use std::process::Command;   
    
fn main(){
	let mut file = File::open("input.txt").expect("Unable to open File");
	let reader = BufReader::new(file); 
	let mut API: String = "https://api.open-meteo.com/v1/forecast?latitude=&longitude=&current_weather=True".to_owned(); 
	let line: String; 

	for line in reader.lines() {
		let pid = unsafe{ fork() }; 
		let data: String = line.expect("Error in data"); 
		let lineSplit = data.split(" "); 
		let dataPoints: Vec<&str> = lineSplit.collect();

		match pid.expect("Fork failed: Unable to create child process"){
			Child => {
				API = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=True", dataPoints[0], dataPoints[0]);   
				let returnValue = Command::new("curl").arg("-o").arg("file").arg(API).output().expect("Fail to execute command"); 
				// println!("{}", API);
				break;  
			}

			Parent {child} => {
				wait(); 

			}


		}  
	} 
 }

