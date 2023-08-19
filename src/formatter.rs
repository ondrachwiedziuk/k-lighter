pub struct HTML {
    content: String,
}

impl HTML {
    pub fn new(bg: String) -> Self {
        let mut text = "<!DOCTYPE html>\n".to_string();
        text.push_str("<document>\n");
        text.push_str(format!("<body style=\"background-color: #{bg};\">\n").as_str());
        Self {content: text}
    }

    pub fn p(&mut self, text: String) {
        self.content.push_str("<p>\n");
        self.content.push_str(text.as_str());
        self.content.push_str("</p>\n");
    }


    pub fn print(&self) -> String {
        let mut text = self.content.clone();
        text.push_str("</body>\n");
        text.push_str("</document>\n");
        return text
    }
}