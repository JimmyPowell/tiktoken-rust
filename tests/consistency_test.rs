use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::Deserialize;
use tiktoken::get_encoding;

#[derive(Deserialize, Debug)]
struct TestCase {
    encoding: String,
    text: String,
    token_count: usize,
}

#[test]
fn test_consistency_with_official_library() {
    let file = File::open("test_cases.jsonl").expect("Failed to open test_cases.jsonl");
    let reader = BufReader::new(file);

    let mut test_count = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }

        let test_case: TestCase = serde_json::from_str(&line)
            .expect(&format!("Failed to parse JSON: {}", line));

        let bpe = get_encoding(&test_case.encoding)
            .expect(&format!("Failed to get encoding: {}", test_case.encoding));

        let rust_token_count = bpe.encode(&test_case.text, &Default::default()).0.len();

        assert_eq!(
            rust_token_count,
            test_case.token_count,
            "Token count mismatch for encoding '{}' and text: '{}'",
            test_case.encoding,
            test_case.text
        );
        test_count += 1;
    }

    assert!(test_count > 0, "No test cases were run!");
    println!("Successfully ran {} consistency test cases.", test_count);
}
