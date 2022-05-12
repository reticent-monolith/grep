pub mod grep {
    pub fn find(pattern: &str, text: &str) -> bool {
        text.contains(pattern)
    }

    pub fn find_ignore_case(pattern: &str, text: &str) -> bool {
        text.to_lowercase().contains(&pattern.to_lowercase())
    }

    #[derive(Debug, PartialEq)]
    pub struct Args {
	pub filename: String,
	pub pattern: String,
	pub ignore: bool
    }
    
    impl Args {
	pub fn parse(cli_args: Vec<String>) -> Result<Self, String> {
	    if cli_args.len() < 3 {
		return Err(String::from("Invalid args, need at least a pattern and file"));
	    }
	    let mut set_ignore = false;
	    if cli_args.len() > 3 && cli_args[3] == "ignore" {
		set_ignore = true;
	    }
	    Ok(Args {
		filename: String::from(&cli_args[2]),
		pattern: String::from(&cli_args[1]),
		ignore: set_ignore
	    })
	}
    }    


    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_find() {
            let tests = [
                ("This is a test sentence.", "test", true),
                ("This is a sentence", "test", false),
                ("This is a Test sentence.", "test", false),
            ];
            for test in &tests {
                assert_eq!(find(test.1, test.0), test.2);
            }
        }

        #[test]
        fn test_find_ignore_case() {
            let tests = [("This is a Test sentence.", "test", true)];
            for test in &tests {
                assert_eq!((find_ignore_case(test.1, test.0)), test.2);
            }
        }

	#[test]
	fn test_Args_parse() {
	    let tests = [
		(vec![
		    "progname".to_string(),
		    "test".to_string(),
		    "filename.txt".to_string()
		],
		Ok(Args{
		    pattern: "test".to_string(),
		    filename: "filename.txt".to_string(),
		    ignore: false
		})),
		(vec![
		    "progname".to_string(),
		    "test".to_string(),
		    "filename.txt".to_string(),
		    "ignore".to_string()
		],
		Ok(Args{
		    pattern: "test".to_string(),
		    filename: "filename.txt".to_string(),
		    ignore: true
		})),
		(vec![
		    "progname".to_string(),
		],
		Err(String::from("Invalid args, need at least a pattern and file"))),
	    ];
	    for (i, test) in tests.iter().enumerate() {
		assert_eq!(Args::parse(test.0.clone()), test.1, "Failed #{}", i+1);
	    }
	}
    }
}
