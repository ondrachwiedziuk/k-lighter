//! This module contains a html frame.

/// Struct containing HTML file content.
/// 
/// Params:
/// 
///     content: String     content of HTML file
pub struct HTML {
    content: String,
}

impl HTML {
    /// Contructor.
    /// Takes a background color and write a header to content.
    /// 
    /// Args:
    ///     bg: String  Name of color for background.
    /// 
    /// Returns:
    /// 
    ///     Self
    pub fn new(bg: String) -> Self {
        let mut text = "<!DOCTYPE html>\n".to_string();
        text.push_str("<document>\n");
        text.push_str(format!("<body style=\"background-color: #{bg};\">\n").as_str());
        Self {content: text}
    }

    /// Writes a paragraph to the content.
    /// 
    /// Args:
    /// 
    ///     text: String    Content of paragraph.
    pub fn p(&mut self, text: String) {
        self.content.push_str("<p>\n");
        self.content.push_str(text.as_str());
        self.content.push_str("</p>\n");
    }


    /// Printing function.
    /// Returns the content as a String.
    /// 
    /// Returns:
    ///     String: HTML.
    pub fn print(&self) -> String {
        let mut text = self.content.clone();
        text.push_str("</body>\n");
        text.push_str("</document>\n");
        return text
    }
}