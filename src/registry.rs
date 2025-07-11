use rustc_hash::FxHashMap as HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::core::CoreBPE;
use crate::loader::load_tiktoken_bpe;
use crate::Rank;

// Using a Mutex to protect the HashMap. This is a simple approach.
// For higher concurrency, one might consider more advanced structures like RwLock
// or concurrent hash maps, but for caching encodings, this is generally sufficient.
static ENCODINGS_CACHE: Lazy<Mutex<HashMap<String, CoreBPE>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

fn cl100k_base() -> Result<CoreBPE, String> {
    let bpe_file = include_str!("../bpe_files/cl100k_base.tiktoken");
    let mergeable_ranks = load_tiktoken_bpe(bpe_file)?;

    let mut special_tokens = HashMap::default();
    special_tokens.insert("<|endoftext|>".to_string(), 100257 as Rank);
    special_tokens.insert("<|fim_prefix|>".to_string(), 100258 as Rank);
    special_tokens.insert("<|fim_middle|>".to_string(), 100259 as Rank);
    special_tokens.insert("<|fim_suffix|>".to_string(), 100260 as Rank);
    special_tokens.insert("<|endofprompt|>".to_string(), 100276 as Rank);

    let pat_str = r#"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+"#;

    CoreBPE::new(
        mergeable_ranks.into_iter().map(|(k, v)| (k, v as Rank)).collect(),
        special_tokens,
        pat_str,
    )
    .map_err(|e| e.to_string())
}

fn o200k_base() -> Result<CoreBPE, String> {
    let bpe_file = include_str!("../bpe_files/o200k_base.tiktoken");
    let mergeable_ranks = load_tiktoken_bpe(bpe_file)?;

    let mut special_tokens = HashMap::default();
    special_tokens.insert("<|endoftext|>".to_string(), 199999 as Rank);
    special_tokens.insert("<|endofprompt|>".to_string(), 200018 as Rank);

    let pat_str = r#"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n/]*|\s*[\r\n]+|\s+(?!\S)|\s+"#;

    CoreBPE::new(
        mergeable_ranks.into_iter().map(|(k, v)| (k, v as Rank)).collect(),
        special_tokens,
        pat_str,
    )
    .map_err(|e| e.to_string())
}

pub fn get_encoding(encoding_name: &str) -> Option<CoreBPE> {
    let mut cache = ENCODINGS_CACHE.lock().unwrap();
    if let Some(encoding) = cache.get(encoding_name) {
        return Some(encoding.clone());
    }

    let constructor = match encoding_name {
        "cl100k_base" => cl100k_base,
        "o200k_base" => o200k_base,
        // Other encodings will be added here
        _ => return None,
    };

    match constructor() {
        Ok(encoding) => {
            cache.insert(encoding_name.to_string(), encoding.clone());
            Some(encoding)
        }
        Err(_) => None,
    }
}
