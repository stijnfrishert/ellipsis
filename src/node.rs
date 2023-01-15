use crate::{utils::sanitize, Color};
use std::io;

pub struct Node {
    pub id: String,
    pub attributes: Vec<NodeAttribute>,
}

impl Node {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            attributes: Vec::new(),
        }
    }

    pub fn label(self, value: impl Into<String>) -> Self {
        self.attribute(NodeAttribute::Label(value.into()))
    }

    pub fn shape(self, value: Option<Shape>) -> Self {
        self.attribute(NodeAttribute::Shape(value))
    }

    pub fn color(self, value: Color) -> Self {
        self.attribute(NodeAttribute::Color(value))
    }

    pub fn attribute(mut self, attribute: NodeAttribute) -> Self {
        self.attributes.push(attribute);

        self
    }

    pub(crate) fn write(&self, mut w: impl io::Write) -> io::Result<()> {
        write!(w, "{}", sanitize(&self.id))?;

        if !self.attributes.is_empty() {
            write!(w, " [")?;

            let mut count = self.attributes.len();
            for attribute in &self.attributes {
                let (key, value) = attribute.pair();
                write!(w, "{}={}", sanitize(key), sanitize(&value))?;

                count -= 1;
                if count > 0 {
                    write!(w, ", ")?;
                }
            }
            write!(w, "]")?;
        }

        Ok(())
    }
}

pub enum NodeAttribute {
    Label(String),
    Shape(Option<Shape>),
    Color(Color),
    Unknown(String, String),
}

impl NodeAttribute {
    pub fn pair(&self) -> (&str, String) {
        match self {
            Self::Label(value) => ("label", value.clone()),
            Self::Shape(value) => (
                "shape",
                match value {
                    Some(shape) => shape.as_str().to_string(),
                    None => String::from("none"),
                },
            ),
            Self::Color(color) => ("color", color.as_string()),
            Self::Unknown(key, value) => (key, value.clone()),
        }
    }
}

pub enum Shape {
    Box,
    Circle,
    Diamond,
    Square,
    Note,
    Unknown(String),
}

impl Shape {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Box => "box",
            Self::Circle => "circle",
            Self::Diamond => "diamond",
            Self::Square => "square",
            Self::Note => "note",
            Self::Unknown(str) => str,
        }
    }
}
