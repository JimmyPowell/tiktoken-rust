# Tiktoken-Rust: A Pure Rust Tokenizer Core

This project is a pure Rust implementation of the core logic from OpenAI's `tiktoken` library. It provides a high-performance, self-contained, and reliable way to handle tokenization for OpenAI models, completely independent of Python.

## Design Philosophy

The primary goal of this refactoring was to create a robust Rust crate that can be easily integrated into any Rust application without external dependencies on Python or network access at runtime.

The key design decision was to **embed BPE data files directly into the library binary at compile time**.

This approach differs from the official Python library (which caches files from the network) and offers several advantages for a compiled language ecosystem:

- **Maximum Reliability**: By including the BPE data in the crate itself, we eliminate an entire class of runtime errors related to network issues, file system permissions, or corrupted cache files. The tokenization data is as reliable as the code itself.
- **Offline Capability**: The compiled library is fully self-contained. It can be deployed and run in any environment, including those that are offline or have restricted network access.
- **Versioning Integrity**: The tokenization rules are immutably tied to a specific version of the library. When OpenAI releases new tokenizers, updating is a standard and predictable process of updating this crate's version, just like any other dependency.

## How to Use

This crate exposes a simple and clear API for tokenization.

### 1. Add to your `Cargo.toml`

```toml
[dependencies]
tiktoken-rust = { git = "https://github.com/JimmyPowell/tiktoken-rust.git" } # Or point to your local path
```

### 2. Basic Usage

The library provides high-level functions to get the correct tokenizer for a given model and then use it to count tokens.

```rust
use tiktoken_rust::{encoding_name_for_model, get_encoding};

fn main() {
    let model = "gpt-4o";
    let text = "Hello world, this is a test sentence for counting tokens.";

    // 1. Get the encoding name for the desired model
    let encoding_name = encoding_name_for_model(model).expect("Model not found");
    
    // 2. Get the tokenizer instance
    let bpe = get_encoding(encoding_name).expect("Encoding not found");

    // 3. Count the tokens
    let token_count = bpe.count_tokens(text);

    println!("Text: \"{}\"", text);
    println!("Token count for model '{}': {}", model, token_count);
    // Output: Token count for model 'gpt-4o': 12
}
```

## Testing and Verification

Ensuring 100% consistency with the official `tiktoken` library was a top priority. We implemented a rigorous, two-stage testing process.

### Stage 1: Unit Tests

We wrote standard unit tests (`cargo test`) to verify the core logic, including:
- Correctly fetching encodings by name (`get_encoding`).
- Correctly mapping model names to encoding names (`encoding_name_for_model`).
- A round-trip test to ensure `decode(encode(text))` perfectly reconstructs the original text.

### Stage 2: Large-Scale Consistency Testing

To validate our implementation against the official Python library on a large scale, we:
1.  **Generated Test Data with AI**: We prompted the Gemini Pro model to generate a diverse set of 50 test cases, including multiple languages, code snippets, special characters, and edge cases.
2.  **Calculated Ground Truth**: We wrote a Python script (`generate_test_cases_with_openai.py`) that used the **official `tiktoken` library** to calculate the "correct" token count for each of the 50 test cases across 2 different encodings (`cl100k_base` and `o200k_base`), resulting in 100 total test vectors.
3.  **Created an Integration Test**: We wrote a Rust integration test (`tests/consistency_test.rs`) that reads the 100 test vectors and asserts that our Rust implementation's output is **identical** to the official library's output for every single case.

This comprehensive testing gives us high confidence in the correctness of this Rust implementation.

## Interactive Example

This project also includes an interactive command-line example to quickly test token counts.

To run it, use `cargo run`:
```sh
cargo run
```
The program will load the `gpt-4o` tokenizer and prompt you to enter text. Type any string and press Enter to see the token count. Type `exit` or `quit` to close the program.

```
$ cargo run
   Finished dev profile [unoptimized + debuginfo] target(s) in 0.27s
    Running `target/debug/example`
Loading tokenizer for model: gpt-4o...
Tokenizer loaded. Enter text to count tokens, or type 'exit' to quit.
> Hello world!
Token count: 2
>
