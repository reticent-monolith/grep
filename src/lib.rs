pub mod grep {
    pub fn find(pattern: &str, text: &str) -> bool {
        text.contains(pattern)
    }

    pub fn find_ignore_case(pattern: &str, text: &str) -> bool {
        text.to_lowercase().contains(&pattern.to_lowercase())
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
    }
}
