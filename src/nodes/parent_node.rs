use super::html_node::{HTMLAttributes, HTMLChildNode, ToHtmlString};

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

    use std::collections::HashMap;

    use crate::nodes::leaf_node::LeafNode;

    use super::*;

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
