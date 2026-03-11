use pyo3::prelude::*;
use pulldown_cmark::{Event, Parser, Tag};

#[pyclass]
pub struct MarkdownSplitter {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

#[pymethods]
impl MarkdownSplitter {
    #[new]
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        MarkdownSplitter {
            chunk_size,
            chunk_overlap,
        }
    }

    /// Splits the markdown `text` into semantic chunks.
    pub fn split_text(&self, text: &str) -> PyResult<Vec<String>> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        
        // Use pulldown_cmark to traverse semantic blocks
        let parser = Parser::new(text);
        
        let mut current_block_start: Option<usize> = None;
        let mut block_depth = 0;

        for (event, range) in parser.into_offset_iter() {
            match event {
                Event::Start(tag) => {
                    // Start of a semantic block
                    if block_depth == 0 {
                        current_block_start = Some(range.start);
                    }
                    block_depth += 1;
                    
                    // If it's a heading, we might want to forcefully break here if the current chunk is large
                    if let Tag::Heading { .. } = tag {
                        if current_chunk.len() >= self.chunk_size / 2 && !current_chunk.trim().is_empty() {
                            chunks.push(current_chunk.trim().to_string());
                            current_chunk.clear();
                        }
                    }
                }
                Event::End(_) => {
                    block_depth -= 1;
                    if block_depth == 0 {
                        // End of a top-level block
                        if let Some(start) = current_block_start {
                            let block_text = &text[start..range.end];
                            
                            // Check if adding this block exceeds chunk_size
                            if current_chunk.len() + block_text.len() > self.chunk_size && !current_chunk.trim().is_empty() {
                                chunks.push(current_chunk.trim().to_string());
                                // Simplistic overlap logic: keep last block (this can be improved)
                                current_chunk.clear();
                            }
                            
                            if !current_chunk.is_empty() {
                                current_chunk.push_str("\n\n");
                            }
                            current_chunk.push_str(block_text);
                        }
                    }
                }
                // Handle text not wrapped in specific blocks (like loose text)
                _ => {}
            }
        }
        
        // Push the last chunk
        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        // Fallback: If empty, maybe the markdown had no blocks (just plain text). 
        // We do a simple character split.
        if chunks.is_empty() && !text.trim().is_empty() {
            let mut start = 0;
            while start < text.len() {
                let end = std::cmp::min(start + self.chunk_size, text.len());
                // find nearest space or newline before `end` to avoid word splitting
                let mut safe_end = end;
                if end < text.len() {
                    for i in (start..end).rev() {
                        if text[i..].starts_with(' ') || text[i..].starts_with('\n') {
                            safe_end = i;
                            break;
                        }
                    }
                    if safe_end == start { safe_end = end; } // No space found
                }
                chunks.push(text[start..safe_end].trim().to_string());
                start = safe_end;
                if self.chunk_overlap > 0 && start > self.chunk_overlap {
                    start -= self.chunk_overlap;
                }
            }
        }

        Ok(chunks)
    }
}

mod ast;
use ast::AstCodeSplitter;

/// A Python module implemented in Rust.
#[pymodule]
fn rust_text_splitter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MarkdownSplitter>()?;
    m.add_class::<AstCodeSplitter>()?;
    Ok(())
}

