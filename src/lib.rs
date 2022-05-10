pub mod grep {
    pub fn find(pattern: &str, text: &str) -> Vec<String> {
        let mut found_lines: Vec<String> = Vec::new();
        for line in text.split('\n') {
            if line.contains(pattern) {
                found_lines.push(String::from(line))
            }
        }
        found_lines
    }

    pub fn find_ignore_case(pattern: &str, text: &str) -> Vec<String> {
        let mut found_lines: Vec<String> = Vec::new();
        for line in text.split('\n') {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                found_lines.push(String::from(line))
            }
        }
        found_lines
    }
}

