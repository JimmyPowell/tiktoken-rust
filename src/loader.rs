use rustc_hash::FxHashMap as HashMap;
use crate::Rank;
use base64::{Engine as _, engine::general_purpose};

/// Parses the contents of a .tiktoken BPE file.
/// The file format is a series of lines, each containing a base64-encoded token
/// followed by a space and its rank.
pub fn load_tiktoken_bpe(bpe_file_contents: &str) -> Result<HashMap<Vec<u8>, Rank>, String> {
    let mut ranks = HashMap::default();
    for line in bpe_file_contents.lines() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(' ');
        let token_b64 = parts.next().ok_or_else(|| format!("Malformed line in BPE file: {}", line))?;
        let rank_str = parts.next().ok_or_else(|| format!("Malformed line in BPE file: {}", line))?;

        let token = general_purpose::STANDARD.decode(token_b64)
            .map_err(|e| format!("Invalid base64 token in BPE file: {}, error: {}", token_b64, e))?;
        
        let rank: Rank = rank_str.parse()
            .map_err(|e| format!("Invalid rank in BPE file: {}, error: {}", rank_str, e))?;

        ranks.insert(token, rank);
    }
    Ok(ranks)
}
