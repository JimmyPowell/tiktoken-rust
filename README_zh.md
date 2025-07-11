# Tiktoken-Rust: 纯 Rust 实现的 Tokenizer 核心

本项目是 OpenAI `tiktoken` 库核心逻辑的纯 Rust 实现。它提供了一种高性能、自包含且可靠的方式来处理 OpenAI 模型的 tokenization，完全独立于 Python。

## 设计思想

本次重构的主要目标是创建一个健壮的 Rust crate，它可以轻松地集成到任何 Rust 应用程序中，并且在运行时不依赖 Python 或网络访问。

核心的设计决策是**在编译时将 BPE 数据文件直接嵌入到库的二进制文件中**。

这种方法与官方 Python 库（从网络缓存文件）不同，并为编译型语言生态系统提供了几个优势：

- **最高的可靠性**: 通过将 BPE 数据包含在 crate 本身中，我们消除了所有与网络问题、文件系统权限或缓存文件损坏相关的运行时错误。Tokenization 数据的可靠性与代码本身一样高。
- **离线能力**: 编译后的库是完全自包含的。它可以在任何环境中部署和运行，包括那些离线或网络访问受限的环境。
- **版本完整性**: Tokenization 规则与库的特定版本不可变地绑定在一起。当 OpenAI 发布新的分词器时，更新过程将是一个标准且可预测的过程——就像更新任何其他依赖项一样，只需更新此 crate 的版本即可。

## 如何使用

本 crate 提供了一个简单清晰的 API 用于 tokenization。

### 1. 添加到您的 `Cargo.toml`

```toml
[dependencies]
tiktoken-rust = { git = "https://github.com/JimmyPowell/tiktoken-rust.git" } # 或指向您的本地路径
```

### 2. 基本用法

本库提供了高级函数，可以为给定模型获取正确的 tokenizer，然后用它来计算 token 数量。

```rust
use tiktoken_rust::{encoding_name_for_model, get_encoding};

fn main() {
    let model = "gpt-4o";
    let text = "你好，世界！这是一个用于计算 token 的测试句子。";

    // 1. 获取所需模型的编码名称
    let encoding_name = encoding_name_for_model(model).expect("未找到模型");
    
    // 2. 获取 tokenizer 实例
    let bpe = get_encoding(encoding_name).expect("未找到编码");

    // 3. 计算 token 数量
    let token_count = bpe.count_tokens(text);

    println!("文本: \"{}\"", text);
    println!("模型 '{}' 的 Token 数量: {}", model, token_count);
}
```

## 测试与验证

确保与官方 `tiktoken` 库 100% 一致是我们的首要任务。我们实施了一个严格的两阶段测试过程。

### 第一阶段：单元测试

我们编写了标准的单元测试 (`cargo test`) 来验证核心逻辑，包括：
- 通过名称正确获取编码 (`get_encoding`)。
- 正确地将模型名称映射到编码名称 (`encoding_name_for_model`)。
- 进行往返测试，以确保 `decode(encode(text))` 能够完美地重构原始文本。

### 第二阶段：大规模一致性测试

为了大规模地验证我们的实现与官方 Python 库的一致性，我们：
1.  **使用 AI 生成测试数据**: 我们请求 Gemini Pro 模型生成了 50 个多样化的测试用例，包括多种语言、代码片段、特殊字符和边界情况。
2.  **计算基准真相 (Ground Truth)**: 我们编写了一个 Python 脚本 (`generate_test_cases_with_openai.py`)，使用**官方 `tiktoken` 库**为这 50 个测试用例在 2 种不同的编码（`cl100k_base` 和 `o200k_base`）下计算“正确”的 token 数量，共产生 100 个测试向量。
3.  **创建集成测试**: 我们编写了一个 Rust 集成测试 (`tests/consistency_test.rs`)，它读取这 100 个测试向量，并断言我们的 Rust 实现的输出与官方库的输出在每一个案例上都**完全相同**。

这种全面的测试使我们对这个 Rust 实现的正确性抱有高度信心。

## 交互式示例

本项目还包含一个交互式的命令行示例，用于快速测试 token 数量。

要运行它，请使用 `cargo run`:
```sh
cargo run
```
程序将加载 `gpt-4o` 的 tokenizer 并提示您输入文本。输入任何字符串并按 Enter 键即可查看 token 数量。输入 `exit` 或 `quit` 可关闭程序。

```
$ cargo run
   Finished dev profile [unoptimized + debuginfo] target(s) in 0.27s
    Running `target/debug/example`
Loading tokenizer for model: gpt-4o...
Tokenizer loaded. Enter text to count tokens, or type 'exit' to quit.
> Hello world!
Token count: 2
> 你好，世界！
Token count: 3
>
