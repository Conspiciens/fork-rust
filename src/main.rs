use std::fs::File;
use std::io::{prelude::*, BufReader};
use nix::sys::wait::wait;
use nix::libc::{perror, execlp};
use nix::unistd::{getpid, fork, ForkResult};
use nix::libc; 
use std::ffi::CString;


fn main(){
	let file = File::open("input.txt").expect("Unable to open File");
	let reader = BufReader::new(file);
	let line: String;
 
	let folder = CString::new("/usr/bin/curl").expect("Fail"); 
	let command = CString::new("curl").expect("Fail");
	let file_input = CString::new("file.json").expect("Faol"); 
	let option = CString::new("-o").expect("Fail");
	let error = CString::new("Error").expect("fail");  		

	for line in reader.lines() {
		let data: String = line.expect("Error in data"); 
		let line_split = data.split(" "); 
		let data_points: Vec<&str> = line_split.collect();

		match unsafe {fork()}{
			Ok(ForkResult::Child) => { 
				let loc_api = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=True", data_points[0], data_points[0]);
				let api = CString::new(loc_api).expect("Fail");
				unsafe {
					let exe_value = execlp(folder.as_ptr(), command.as_ptr(), option.as_ptr(), file_input.as_ptr(), api.as_ptr(), std::ptr::null::<*const libc::c_char>());
 
					if exe_value < 0 {
						perror(error.as_ptr()); 
						panic!("Error occured {}", exe_value);
					} 
				}
			 }
 
			Ok(ForkResult::Parent {child}) => {
				wait().expect("Error");
				println!("Hello from parent {}", getpid());  

			}

			Err(_) => println!("Fork Failed"), 
		}
	} 
 }

