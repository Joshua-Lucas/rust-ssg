use core::fmt;

#[derive(PartialEq)]
pub struct TextNode {
    pub content: String,
    pub text_type: TextType,
    pub url: Option<String>,
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = match &self.url {
            None => "",
            Some(x) => &x,
        };

        write!(
            f,
            "TextNode({}, {:?}, {})",
            self.content, self.text_type, url
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum TextType {
    Normal,
    Bold,
    Italic,
    Code,
    Links,
    Images,
}
