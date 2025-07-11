use std::io::{self, Write};
use tiktoken::{encoding_name_for_model, get_encoding};

fn main() {
    let model = "gpt-4o";
    println!("Loading tokenizer for model: {}...", model);
    
    let encoding_name = encoding_name_for_model(model).expect("Model not found");
    let bpe = get_encoding(encoding_name).expect("Encoding not found");
    
    println!("Tokenizer loaded. Enter text to count tokens, or type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (e.g., Ctrl+D)
                println!("\nExiting.");
                break;
            }
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.eq_ignore_ascii_case("exit") || trimmed_input.eq_ignore_ascii_case("quit") {
                    println!("Exiting.");
                    break;
                }

                let token_count = bpe.count_tokens(trimmed_input);
                println!("Token count: {}", token_count);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}
