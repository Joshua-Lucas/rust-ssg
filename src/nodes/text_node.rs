use core::fmt;
use std::collections::HashMap;

use super::html_node::HTMLAttributes;
use super::leaf_node::LeafNode;

#[derive(Debug, PartialEq, Clone)]
pub enum TextType {
    Normal,
    Bold,
    Italic,
    Code,
    Link,
    Image,
}

#[derive(Debug)]
struct MarkdownDelimiter {
    name: TextType,
    delimiters: &'static [&'static str],
}

impl TextType {
    fn get_delimiter(&self) -> Option<MarkdownDelimiter> {
        match self {
            TextType::Normal => None,
            TextType::Bold => Some(MarkdownDelimiter {
                name: TextType::Bold,
                delimiters: &["**", "__"],
            }),
            TextType::Italic => Some(MarkdownDelimiter {
                name: TextType::Italic,
                delimiters: &["*", "_"],
            }),
            TextType::Code => Some(MarkdownDelimiter {
                name: TextType::Code,
                delimiters: &["`"],
            }),
            // todo!() refactor later
            TextType::Link => None,
            TextType::Image => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn split_node_on_delimiter(&self, delimiter: MarkdownDelimiter) -> Vec<TextNode> {
        let text_nodes = match self.text_type {
            TextType::Normal => {
                let content = vec![self.content.as_str()];

                let delim = &delimiter.delimiters;
                // Update this to convert strings into text nodes.
                // Then also have to account for there being both text node
                // type and str type maybe us a generic ?
                let split = delim.into_iter().fold(content, |acc, d| {
                    acc.iter()
                        .flat_map(|&x| x.split(d).collect::<Vec<_>>())
                        .collect()
                });

                // let deliminators = vec![TextType::Bold, TextType::Italic, TextType::Code];

                // deliminators.iter().fold(vec![self.content], |acc, delim| {
                //     let type_deliminators = delim.get_delimiter().unwrap();

                //     let delm_vec = type_deliminators.delimiters.iter().fold(&acc, |x, d| {
                //         x.into_iter().reduce(|acc, a| acc.split(*d)).unwrap()
                //     });
                //     println!("{:?}", type_deliminators);
                //     println!("{:?}", delm_vec);
                //     acc
                // });

                // Determine deliminator. Need to loop through all delimnators
                // Maybe create an enum maybe a hashmap for deliminator types.
                // like bold = ** or __
                // like HashMap::from({"**": TextType::Bold, "__": TextType::Bold})
                vec![TextNode {
                    content: String::from("test"),
                    url: None,
                    text_type: TextType::Normal,
                }]
            }
            _ => vec![self.clone()],
        };

        return text_nodes;
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

    #[test]
    fn parse_deliminator() {}
}
