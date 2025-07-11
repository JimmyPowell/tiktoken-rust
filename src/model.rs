use phf::phf_map;

static MODEL_PREFIX_TO_ENCODING: phf::Map<&'static str, &'static str> = phf_map! {
    "o1-" => "o200k_base",
    "o3-" => "o200k_base",
    // chat
    "chatgpt-4o-" => "o200k_base",
    "gpt-4o-" => "o200k_base",
    "gpt-4-" => "cl100k_base",
    "gpt-3.5-turbo-" => "cl100k_base",
    "gpt-35-turbo-" => "cl100k_base",
    // fine-tuned
    "ft:gpt-4o" => "o200k_base",
    "ft:gpt-4" => "cl100k_base",
    "ft:gpt-3.5-turbo" => "cl100k_base",
    "ft:davinci-002" => "cl100k_base",
    "ft:babbage-002" => "cl100k_base",
};

static MODEL_TO_ENCODING: phf::Map<&'static str, &'static str> = phf_map! {
    // reasoning
    "o1" => "o200k_base",
    "o3" => "o200k_base",
    // chat
    "gpt-4o" => "o200k_base",
    "gpt-4" => "cl100k_base",
    "gpt-3.5-turbo" => "cl100k_base",
    "gpt-3.5" => "cl100k_base",
    "gpt-35-turbo" => "cl100k_base",
    // base
    "davinci-002" => "cl100k_base",
    "babbage-002" => "cl100k_base",
    // embeddings
    "text-embedding-ada-002" => "cl100k_base",
    "text-embedding-3-small" => "cl100k_base",
    "text-embedding-3-large" => "cl100k_base",
    // DEPRECATED MODELS
    "text-davinci-003" => "p50k_base",
    "text-davinci-002" => "p50k_base",
    "text-davinci-001" => "r50k_base",
    "text-curie-001" => "r50k_base",
    "text-babbage-001" => "r50k_base",
    "text-ada-001" => "r50k_base",
    "davinci" => "r50k_base",
    "curie" => "r50k_base",
    "babbage" => "r50k_base",
    "ada" => "r50k_base",
    "code-davinci-002" => "p50k_base",
    "code-davinci-001" => "p50k_base",
    "code-cushman-002" => "p50k_base",
    "code-cushman-001" => "p50k_base",
    "davinci-codex" => "p50k_base",
    "cushman-codex" => "p50k_base",
    "text-davinci-edit-001" => "p50k_edit",
    "code-davinci-edit-001" => "p50k_edit",
    "text-similarity-davinci-001" => "r50k_base",
    "text-similarity-curie-001" => "r50k_base",
    "text-similarity-babbage-001" => "r50k_base",
    "text-similarity-ada-001" => "r50k_base",
    "text-search-davinci-doc-001" => "r50k_base",
    "text-search-curie-doc-001" => "r50k_base",
    "text-search-babbage-doc-001" => "r50k_base",
    "text-search-ada-doc-001" => "r50k_base",
    "code-search-babbage-code-001" => "r50k_base",
    "code-search-ada-code-001" => "r50k_base",
    "gpt2" => "gpt2",
    "gpt-2" => "gpt2",
};

/// Returns the name of the encoding used by a model.
pub fn encoding_name_for_model(model_name: &str) -> Result<&'static str, &'static str> {
    if let Some(encoding_name) = MODEL_TO_ENCODING.get(model_name) {
        return Ok(encoding_name);
    }

    for (model_prefix, model_encoding_name) in MODEL_PREFIX_TO_ENCODING.entries() {
        if model_name.starts_with(model_prefix) {
            return Ok(model_encoding_name);
        }
    }

    Err("Could not automatically map model name to a tokeniser.")
}
