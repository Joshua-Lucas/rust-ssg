use core::fmt;
use std::collections::HashMap;

use super::leaf_node::LeafNode;

// HTML Attribute type is used throughout all the nodes.
#[derive(Debug, PartialEq)]
pub struct HTMLAttributes {
    pub attr: HashMap<String, String>,
}

impl HTMLAttributes {
    // Converts the Hash map into an HTML string
    pub fn to_html(&self) -> String {
        // I want the attributes to produce a lexicographical string, so I
        // extract the keys, sort then concat the strings.
        let mut keys: Vec<&String> = self.attr.keys().collect();

        keys.sort();

        let string_vec: Vec<String> = keys
            .iter()
            .map(|&key| format!("{}=\"{}\"", key, self.attr[key]))
            .collect();

        return string_vec.join(" ");
    }
}

#[derive(Debug, PartialEq)]
pub enum HTMLChildNode {
    HTML(HTMLNode),
    Leaf(LeafNode),
    Parent(ParentNode),
}

pub trait ToHtmlString {
    fn into_html(&self) -> String;
}

impl ToHtmlString for HTMLChildNode {
    fn into_html(&self) -> String {
        match self {
            HTMLChildNode::HTML(x) => x.into_html(),
            HTMLChildNode::Leaf(x) => x.into_html(),
            HTMLChildNode::Parent(x) => x.into_html(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HTMLNode {
    pub tag: Option<String>,
    pub value: Option<String>,
    pub children: Option<Vec<HTMLNode>>,
    pub attributes: Option<HTMLAttributes>,
}

impl ToHtmlString for HTMLNode {
    //todo! update later to turn the HTMLNode into html
    fn into_html(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for HTMLNode {
    // This is for debugging purposes
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = match &self.tag {
            None => "",
            Some(t) => t,
        };

        let value = match &self.value {
            None => "",
            Some(v) => v,
        };

        let children = match &self.children {
            None => String::from(""),
            Some(c) => c
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(" "),
        };

        let attr = match &self.attributes {
            Some(a) => a.to_html(),
            None => String::from(""),
        };

        write!(
            f,
            "HtmlNode({:?}, {:?}, {:?},{:?})",
            tag, value, children, attr
        )
    }
}

// Parent Node will handle the nesting of html nodes.
#[derive(Debug, PartialEq)]
pub struct ParentNode {
    tag: String,
    children: Vec<HTMLChildNode>,
    attributes: Option<HTMLAttributes>,
}

impl ToHtmlString for ParentNode {
    fn into_html(&self) -> String {
        // Take tag and attributes and get tag with
        let opening_parent_tag = match &self.attributes {
            Some(a) => format!("{} {}", self.tag, a.to_html()),
            None => format!("{}", self.tag),
        };

        // Loop through children
        let hmtl_strings: Vec<String> = self.children.iter().map(|c| c.into_html()).collect();

        return format!(
            "<{}>{}</{}>",
            opening_parent_tag,
            hmtl_strings.join(""),
            self.tag
        );
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Test for HTMLAttributes struct
    #[test]
    fn test_html_attribute_to_string() {
        let test_cases = vec![
            (
                "Test One Attribute to html string",
                HTMLAttributes {
                    attr: HashMap::from([
                        (String::from("href"), String::from("https://www.google.com")),
                        (String::from("target"), String::from("_blank")),
                    ]),
                },
                String::from("href=\"https://www.google.com\" target=\"_blank\""),
            ),
            (
                "Test multiple attributes to html string",
                HTMLAttributes {
                    attr: HashMap::from([(
                        String::from("href"),
                        String::from("https://www.google.com"),
                    )]),
                },
                String::from("href=\"https://www.google.com\""),
            ),
        ];

        for (title, value, expected) in test_cases.iter() {
            assert_eq!(
                &value.to_html(),
                expected,
                "\"{}\" test failed for input: {:?} and expexted: {}",
                title,
                value,
                expected
            );
        }
    }

    #[test]
    fn test_parent_node_to_html() {
        let test_cases = vec![(
                "Test parent with no attr to html string",
                ParentNode {
                    tag: String::from("p"),
                    children: vec![
                        HTMLChildNode::Leaf(LeafNode {
                            tag: Some(String::from("b")),
                            value: String::from("Bold text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: None,
                            value: String::from("Normal text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: Some(String::from("i")),
                            value: String::from("italic text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: None,
                            value: String::from("Normal text"),
                            attributes: None,
                        }),
                    ],
                    attributes: None,
                },
                String::from("<p><b>Bold text</b>Normal text<i>italic text</i>Normal text</p>"),
        ),(
            "Test parent with attributes to html string",
            ParentNode {
                    tag: String::from("a"),
                    children: vec![
                        HTMLChildNode::Leaf(LeafNode {
                            tag: Some(String::from("b")),
                            value: String::from("Bold text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: None,
                            value: String::from("Normal text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: Some(String::from("i")),
                            value: String::from("italic text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: None,
                            value: String::from("Normal text"),
                            attributes: None,
                        }),
                    ],
                    attributes: Some(HTMLAttributes {
                        attr: HashMap::from([
                            (String::from("href"), String::from("https://www.google.com")),
                            (String::from("target"), String::from("_blank")),
                        ]),
                }),
                },
                String::from("<a href=\"https://www.google.com\" target=\"_blank\"><b>Bold text</b>Normal text<i>italic text</i>Normal text</a>"),
            ),
            ("Test parent with parent to html string",
                ParentNode {
                    tag: String::from("p"),
                    children: vec![
                        HTMLChildNode::Leaf(LeafNode {
                            tag: Some(String::from("b")),
                            value: String::from("Bold text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Parent(ParentNode {
                            tag: String::from("a"),
                            children: vec![
                                HTMLChildNode::Leaf(LeafNode {
                                    tag: None,
                                    value: String::from("Anchor text"),
                                    attributes: None,
                                }),
                            ],
                            attributes: Some(HTMLAttributes {
                                attr: HashMap::from([
                                    (String::from("href"), String::from("https://www.google.com")),
                                    (String::from("target"), String::from("_blank")),
                                ]),
                            }),
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: Some(String::from("i")),
                            value: String::from("italic text"),
                            attributes: None,
                        }),
                        HTMLChildNode::Leaf(LeafNode {
                            tag: None,
                            value: String::from("Normal text"),
                            attributes: None,
                        }),
                    ],
                    attributes: None,
                },
                String::from("<p><b>Bold text</b><a href=\"https://www.google.com\" target=\"_blank\">Anchor text</a><i>italic text</i>Normal text</p>"),
        )];

        for (title, input, expected) in test_cases.iter() {
            assert_eq!(
                &input.into_html(),
                expected,
                "\"{}\" test failed for input: {:?} and expexted: {}",
                title,
                input,
                expected
            );
        }
    }
}
