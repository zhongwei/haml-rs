use ast::{Attribute, Attributes, Html};

pub struct Span {
    children: Option<Vec<Box<dyn Html>>>,
    attributes: Option<Attributes>,
    tag: Option<String>,
}

impl Html for Span {
    fn tag(&self) -> &Option<String> {
        &self.tag
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(
                vec![child]
            );
        }
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        if let Some(ref mut attributes) = self.attributes {
            attributes.push(attribute);
        } else {
            self.attributes = Some(
                vec![ attribute]
            );
        }

    }

    fn children(&self) -> &Option<Vec<Box<dyn Html>>> {
        &self.children
    }

    fn attributes(&self) -> &Option<Attributes> {
        &self.attributes
    }
}

impl Span {
    pub fn new() -> Span {
        Span {
            attributes: None,
            children: None,
            tag: Some("span".to_string()),
        }
    }

    pub fn boxed() -> Box<Self> {
        Box::new(
            Span {
                attributes: None,
                children: None,
                tag: Some("span".to_string()),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::ast::Text;

    #[test]
    fn test_basic_span() {
        let span = Span::new();
        assert_eq!("<span></span>", span.to_html());
    }

    #[test]
    fn test_span_with_inner_text() {
        let expected_text = "This is a test";
        let text = Text::new(expected_text.to_string());
        let mut span = Span::new();
        span.add_child(Box::new(text));
        assert_eq!(format!("<span>{}</span>", expected_text), span.to_html());
    }

    #[test]
    fn test_span_with_child() {
        let mut parent = Span::new();
        let child = Span::boxed();
        parent.add_child(child);
        assert_eq!("<span><span></span></span>", parent.to_html());
    }
}