use grep::grep::*;

#[test]
fn test_find() {
    let tests = [
        (
            "This is a test sentence.\n",
            "test",
            vec!["This is a test sentence."],
        ),
        (
            "This is a test sentence.\nAnd this is another test sentence.\n",
            "test",
            vec![
                "This is a test sentence.",
                "And this is another test sentence.",
            ],
        ),
        ("There should be no match here", "test", vec![]),
        (
            "This is a test sentence.\nThis is a sentence without a match.\n",
            "test",
            vec!["This is a test sentence."],
        ),
        (
            "This is a test sentence.\nThis is another Test sentence.\n",
            "test",
            vec!["This is a test sentence."],
        ),
    ];
    for test in &tests {
        assert_eq!(find(test.1, test.0), test.2);
    }
}

#[test]
fn test_find_ignore_case() {
    let tests = [(
        "This is a test sentence.\nThis is another Test sentence.\n",
        "test",
        vec!["This is a test sentence.", "This is another Test sentence."],
    )];
    for test in &tests {
        assert_eq!((find_ignore_case(test.1, test.0)), test.2);
    }
}
