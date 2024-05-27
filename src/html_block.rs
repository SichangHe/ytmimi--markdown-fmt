use super::*;

/// A buffer where we write HTML blocks. Preserves everything as is.
pub struct PreservingHtmlBlock {
    buffer: String,
}

impl Write for PreservingHtmlBlock {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl ParagraphFormatter for PreservingHtmlBlock {
    fn new(_max_width: Option<usize>, capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn into_buffer(self) -> String {
        self.buffer
    }
}
