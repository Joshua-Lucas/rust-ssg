use core::fmt;
use std::collections::HashMap;

// HTML Attribute type is used throughout all the nodes.
#[derive(Debug, PartialEq)]
pub struct HTMLAttributes {
    pub attr: HashMap<String, String>,
}

impl HTMLAttributes {
    // Converts the Hash map into an HTML string
    fn to_html(&self) -> String {
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

trait ToHtmlString {
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

// Leaf Node is a type of HTMLNode that represents a single HTML tag with no
// children.
#[derive(Debug, PartialEq)]
pub struct LeafNode {
    pub tag: Option<String>,
    pub value: String,
    pub attributes: Option<HTMLAttributes>,
}

impl ToHtmlString for LeafNode {
    // Takes the leaf node an turns it into an html string.
    fn into_html(&self) -> String {
        // If there is a tag then should wrap the value in the tag, but when
        // there is no tag should return raw text.
        match &self.tag {
            Some(t) => {
                if let Some(a) = &self.attributes {
                    format!("<{} {}>{}</{}>", t, a.to_html(), self.value, t)
                } else {
                    // Ex. t = p then "<p class="disabled">This is the value</p>
                    format!("<{}>{}</{}>", t, self.value, t)
                }
            }
            None => self.value.clone(),
        }
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

    // Tests for LeafNode
    #[test]
    fn test_new_leaf_node() {}

    #[test]
    fn test_leaf_node_to_html() {
        let test_cases = vec![
            (
                "Test Leaf Node with a tag",
                LeafNode {
                    tag: Some(String::from("p")),
                    value: String::from("This is a paragraph of text."),
                    attributes: None,
                },
                String::from("<p>This is a paragraph of text.</p>"),
            ),
            (
                "Test Leaf Node without a tag",
                LeafNode {
                    tag: None,
                    value: String::from("This is plain text."),
                    attributes: None,
                },
                String::from("This is plain text."),
            ),
            (
                "Test Leaf Node with tag and attributes",
                LeafNode {
                    tag: Some(String::from("a")),
                    value: String::from("Click me!"),
                    attributes: Some(HTMLAttributes {
                        attr: HashMap::from([
                            (String::from("href"), String::from("https://www.google.com")),
                            (String::from("target"), String::from("_blank")),
                        ]),
                    }),
                },
                String::from("<a href=\"https://www.google.com\" target=\"_blank\">Click me!</a>"),
            ),
        ];

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
