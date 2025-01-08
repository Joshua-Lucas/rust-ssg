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
}
