use grep::grep;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), String> {
    // get pattern and filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
	return Err(String::from("Invalid args, need at least a pattern and file"));
    }
    let mut ignore = false;
    if args.len() > 3 && args[3] == "ignore" {
	ignore = true;
    }
    // use a function pointer to allow ignore case to call different fn
    let mut func: fn(&str, &str) -> bool = grep::find;
    if ignore { func = grep::find_ignore_case; }
    // read file to a string or set up an iterator
    let file = match fs::File::open(&args[2]) {
	Ok(file) => file,
	Err(e) => {return Err(e.to_string());}
    };
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    loop {
	match lines.next() {
	    Some(line) => {
		let unwrapped = &line.unwrap();
		if func(&args[1], &unwrapped) {
		    println!("{}", &unwrapped)
		}
	    },
	    None => break
	}
    }
    Ok(())
}
