use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HTMLNode {
    pub tag: Option<String>,
    pub value: Option<String>,
    pub children: Option<Vec<HTMLNode>>,
    pub attributes: Option<HashMap<String, String>>,
}

impl HTMLNode {
    //todo! update later to turn the HTMLNode into html
    fn into_html(self) {}

    /// Converts the attributes HashMap, if Some, to an HTML attribute string.
    /// Otherwise it returns an empty String.
    pub fn attr_to_html(&self) -> String {
        if let Some(attr) = &self.attributes {
            // I want the attributes to produce a lexicographical string, so I
            // extract the keys, sort then concat the strings.
            let mut keys: Vec<&String> = attr.keys().collect();

            keys.sort();

            let string_vec: Vec<String> = keys
                .iter()
                .map(|&key| format!("{}=\"{}\"", key, attr[key]))
                .collect();

            return string_vec.join(" ");
        } else {
            return String::from("");
        }
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

        let attr = self.attr_to_html();

        write!(
            f,
            "HtmlNode({:?}, {:?}, {:?},{:?})",
            tag, value, children, attr
        )
    }
}

// Leaf Node is a type of HTMLNode that represents a single HTML tag with no
// children.
#[derive(Debug)]
pub struct LeafNode {
    pub tag: Option<String>,
    pub value: String,
    pub attributes: Option<HashMap<String, String>>,
}

impl LeafNode {
    fn new(
        tag: Option<String>,
        value: String,
        attributes: Option<HashMap<String, String>>,
    ) -> Result<Self, String> {
        if value.trim().is_empty() {
            Err(String::from("LeafNode must contain a value"))
        } else {
            Ok(Self {
                tag,
                value,
                attributes,
            })
        }
    }

    // Takes the leaf node an turns it into an html string.
    fn to_html(&self) -> String {
        // If there is a tag then should wrap the value in the tag, but when
        // there is no tag should return raw text.
        match &self.tag {
            Some(t) => {
                if let Some(_) = &self.attributes {
                    format!("<{} {}>{}</{}>", t, self.attr_to_html(), self.value, t)
                } else {
                    // Ex. t = p then "<p class="disabled">This is the value</p>
                    format!("<{}>{}</{}>", t, self.value, t)
                }
            }
            None => self.value.clone(),
        }
    }

    // Converts the attributes HashMap, if Some, to an HTML attribute string.
    // Otherwise it returns an empty String.
    pub fn attr_to_html(&self) -> String {
        if let Some(attr) = &self.attributes {
            // I want the attributes to produce a lexicographical string, so I
            // extract the keys, sort then concat the strings.
            let mut keys: Vec<&String> = attr.keys().collect();

            keys.sort();

            let string_vec: Vec<String> = keys
                .iter()
                .map(|&key| format!("{}=\"{}\"", key, attr[key]))
                .collect();

            return string_vec.join(" ");
        } else {
            return String::from("");
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_attribute_hash_to_string() {
        let test_cases = vec![
            (
                "Test Node has attributes",
                HTMLNode {
                    tag: Some(String::from("p")),
                    value: Some(String::from("This is a Test Node")),
                    children: None,
                    attributes: Some(HashMap::from([
                        (String::from("href"), String::from("https://www.google.com")),
                        (String::from("target"), String::from("_blank")),
                    ])),
                },
                String::from("href=\"https://www.google.com\" target=\"_blank\""),
            ),
            (
                "Test Node that has no attributes",
                HTMLNode {
                    tag: Some(String::from("p")),
                    value: Some(String::from("This is a Test Node")),
                    children: None,
                    attributes: None,
                },
                String::from(""),
            ),
        ];

        for (title, value, expected) in test_cases.iter() {
            assert_eq!(
                &value.attr_to_html(),
                expected,
                "\"{}\" test failed for input: {} and expexted: {}",
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
                String::from("<p>This is a paragraph of text</p>"),
            ),
            (
                "Test Leaf Node without a tag",
                LeafNode {
                    tag: None,
                    value: String::from("This is plain text."),
                    attributes: None,
                },
                String::from("This is plain text"),
            ),
            (
                "Test Leaf Node with tag and attributes",
                LeafNode {
                    tag: Some(String::from("a")),
                    value: String::from("Click me!"),
                    attributes: Some(HashMap::from([
                        (String::from("href"), String::from("https://www.google.com")),
                        (String::from("target"), String::from("_blank")),
                    ])),
                },
                String::from("<a href=\"https://www.google.com\" target=\"_blank\">Click me!</a>"),
            ),
        ];

        for (title, input, expected) in test_cases.iter() {
            assert_eq!(
                &input.to_html(),
                expected,
                "\"{}\" test failed for input: {:?} and expexted: {}",
                title,
                input,
                expected
            );
        }
    }
}
