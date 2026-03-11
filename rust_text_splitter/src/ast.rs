use pyo3::prelude::*;
use tree_sitter::Parser;

#[pyclass]
pub struct AstCodeSplitter {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

#[pymethods]
impl AstCodeSplitter {
    #[new]
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        AstCodeSplitter {
            chunk_size,
            chunk_overlap,
        }
    }

    /// Splits the Python `code` into semantic chunks using AST parsing.
    pub fn split_text(&self, code: &str) -> PyResult<Vec<String>> {
        let mut parser = Parser::new();
        // Set the language to Python
        parser.set_language(&tree_sitter_python::LANGUAGE.into()).expect("Error loading Python grammar");

        let tree = parser.parse(code, None).expect("Error parsing code");
        let root_node = tree.root_node();

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        // We iterate through the top-level nodes of the AST
        let mut cursor = root_node.walk();
        for child in root_node.children(&mut cursor) {
            let node_text = &code[child.start_byte()..child.end_byte()];
            
            // If it's a major definition (function, class), we might want to prioritize keeping it together
            match child.kind() {
                "function_definition" | "class_definition" | "decorated_definition" => {
                    // Start a new chunk if current is getting full and not empty
                    if current_chunk.len() >= self.chunk_size / 2 && !current_chunk.is_empty() {
                        chunks.push(current_chunk.trim().to_string());
                        current_chunk.clear();
                    }
                    
                    if !current_chunk.is_empty() {
                         current_chunk.push_str("\n\n");
                    }
                    current_chunk.push_str(node_text);

                    // If the node itself is huge, we might exceed chunk_size, but we prioritize AST integrity
                    if current_chunk.len() >= self.chunk_size {
                        chunks.push(current_chunk.trim().to_string());
                        current_chunk.clear();
                    }
                }
                _ => {
                    // For typical statements or imports
                    if current_chunk.len() + node_text.len() > self.chunk_size && !current_chunk.is_empty() {
                        chunks.push(current_chunk.trim().to_string());
                        current_chunk.clear();
                    }
                    
                    if !current_chunk.is_empty() {
                         current_chunk.push_str("\n");
                    }
                    current_chunk.push_str(node_text);
                }
            }
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        // Fallback for empty chunks but non-empty input
        if chunks.is_empty() && !code.trim().is_empty() {
            chunks.push(code.trim().to_string());
        }

        Ok(chunks)
    }
}
