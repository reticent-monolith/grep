use grep::grep;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), String> {
    let args = grep::Args::parse(env::args().collect())?;
    let mut func: fn(&str, &str) -> bool = grep::find;
    if args.ignore { func = grep::find_ignore_case; }
    let file = match fs::File::open(args.filename) {
	Ok(file) => file,
	Err(e) => {return Err(e.to_string());}
    };
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    loop {
	match lines.next() {
	    Some(line) => {
		let unwrapped = &line.unwrap();
		if func(&args.pattern, &unwrapped) {
		    println!("{}", &unwrapped)
		}
	    },
	    None => break
	}
    }
    Ok(())
}
