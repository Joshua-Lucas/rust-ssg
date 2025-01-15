use super::html_node::{HTMLAttributes, ToHtmlString};

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
mod test {
    use std::collections::HashMap;

    use super::*;

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
