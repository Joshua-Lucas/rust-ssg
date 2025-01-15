use core::fmt;
use std::collections::HashMap;

use super::html_node::HTMLAttributes;
use super::leaf_node::LeafNode;

#[derive(Debug, PartialEq)]
pub enum TextType {
    Normal,
    Bold,
    Italic,
    Code,
    Link,
    Image,
}

#[derive(Debug, PartialEq)]
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

impl TextNode {
    pub fn into_html_node(&self) -> LeafNode {
        match self.text_type {
            TextType::Normal => LeafNode {
                tag: None,
                value: self.content.clone(),
                attributes: None,
            },
            TextType::Bold => LeafNode {
                tag: Some(String::from("b")),
                value: self.content.clone(),
                attributes: None,
            },
            TextType::Italic => LeafNode {
                tag: Some(String::from("i")),
                value: self.content.clone(),
                attributes: None,
            },
            TextType::Code => LeafNode {
                tag: Some(String::from("code")),
                value: self.content.clone(),
                attributes: None,
            },
            TextType::Link => {
                let url = self.url.clone().unwrap_or(String::from(""));

                LeafNode {
                    tag: Some(String::from("a")),
                    value: self.content.clone(),
                    attributes: Some(HTMLAttributes {
                        attr: HashMap::from([(String::from("href"), url)]),
                    }),
                }
            }
            TextType::Image => {
                let url = self.url.clone().unwrap_or(String::from(""));

                LeafNode {
                    tag: Some(String::from("img")),
                    value: String::from(""),
                    attributes: Some(HTMLAttributes {
                        attr: HashMap::from([
                            (String::from("href"), url),
                            (String::from("alt"), self.content.clone()),
                        ]),
                    }),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_text_node_into_html_node() {
        let test_cases = vec![
            (
                "Test normal text text node type",
                TextNode {
                    content: String::from("Normal Text Type"),
                    text_type: TextType::Normal,
                    url: None,
                },
                LeafNode {
                    tag: None,
                    value: String::from("Normal Text Type"),
                    attributes: None,
                },
            ),
            (
                "Test bold text node type",
                TextNode {
                    content: String::from("Bold Text Type"),
                    text_type: TextType::Bold,
                    url: None,
                },
                LeafNode {
                    tag: Some(String::from("b")),
                    value: String::from("Bold Text Type"),
                    attributes: None,
                },
            ),
            (
                "Test italic text node type",
                TextNode {
                    content: String::from("Italic Text Type"),
                    text_type: TextType::Italic,
                    url: None,
                },
                LeafNode {
                    tag: Some(String::from("i")),
                    value: String::from("Italic Text Type"),
                    attributes: None,
                },
            ),
            (
                "Test code node type",
                TextNode {
                    content: String::from("const code = \"foo\""),
                    text_type: TextType::Code,
                    url: None,
                },
                LeafNode {
                    tag: Some(String::from("code")),
                    value: String::from("const code = \"foo\""),
                    attributes: None,
                },
            ),
            (
                "Test anchor node type",
                TextNode {
                    content: String::from("About Us"),
                    text_type: TextType::Link,
                    url: Some(String::from("https://google.com")),
                },
                LeafNode {
                    tag: Some(String::from("a")),
                    value: String::from("About Us"),
                    attributes: Some(HTMLAttributes {
                        attr: HashMap::from([(
                            String::from("href"),
                            String::from("https://google.com"),
                        )]),
                    }),
                },
            ),
            (
                "Test image node type",
                TextNode {
                    content: String::from("Two kittens playing with yarn"),
                    text_type: TextType::Image,
                    url: Some(String::from("https://placeholder.cdn.com")),
                },
                LeafNode {
                    tag: Some(String::from("img")),
                    value: String::from(""),
                    attributes: Some(HTMLAttributes {
                        attr: HashMap::from([
                            (
                                String::from("href"),
                                String::from("https://placeholder.cdn.com"),
                            ),
                            (
                                String::from("alt"),
                                String::from("Two kittens playing with yarn"),
                            ),
                        ]),
                    }),
                },
            ),
        ];

        for (title, input, expected) in test_cases.iter() {
            assert_eq!(
                &input.into_html_node(),
                expected,
                "\"{}\" test failed for input: {:?} and expexted: {:?}",
                title,
                input,
                expected
            );
        }
    }
}
