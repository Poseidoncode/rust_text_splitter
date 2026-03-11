# rust_text_splitter

`rust_text_splitter` 是一個高效能的文本分割工具，基於 Rust 開發，並提供 Python 綁定。它支援語義化的 Markdown 分割以及基於 AST (Tree-Sitter) 的程式碼分割（目前支援 Python）。

## 特色

- **高效能**: 使用 Rust 核心實作，適合處理大規模數據。
- **語義化 Markdown 分割**: 能夠識別 Markdown 標題與標籤，保持上下文完整。
- **AST 代碼分割**: 利用 Tree-Sitter 解析程式碼結構，精確地按函數或類別分割程式碼。

## 安裝

```bash
pip install rust_text_splitter
```

## 使用範例

### Markdown 分割

```python
from rust_text_splitter import MarkdownSplitter

splitter = MarkdownSplitter(chunk_size=1000, chunk_overlap=200)
text = "# Heading\nSome content here..."
chunks = splitter.split_text(text)

for chunk in chunks:
    print(chunk)
```

### AST 代碼分割 (Python)

```python
from rust_text_splitter import AstCodeSplitter

splitter = AstCodeSplitter()
code = """
def hello():
    print("Hello world")

class MyClass:
    pass
"""
chunks = splitter.split_text(code)

for chunk in chunks:
    print(chunk)
```

## 開發

### 建置

本專案使用 [maturin](https://github.com/PyO3/maturin) 進行開發：

```bash
# 安裝開發版本
maturin develop
```

### 測試

```bash
# Rust 測試
cargo test

# Python 測試
pytest tests/
```

## 授權

[Apache-2.0](LICENSE)
