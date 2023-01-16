use crate::{utils::sanitize, Color, Label};
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

    pub fn label(self, value: impl Into<Label>) -> Self {
        self.attribute(NodeAttribute::Label(value.into()))
    }

    pub fn shape(self, value: Option<Shape>) -> Self {
        self.attribute(NodeAttribute::Shape(value))
    }

    /// The color of the outline
    pub fn color<V: TryInto<Color>>(self, value: V) -> Result<Self, <V>::Error> {
        let color = value.try_into()?;
        Ok(self.attribute(NodeAttribute::Color(color)))
    }

    pub fn fill_color<V: TryInto<Color>>(self, value: V) -> Result<Self, <V>::Error> {
        let color = value.try_into()?;
        Ok(self.attribute(NodeAttribute::FillColor(color)))
    }

    pub fn font_color<V: TryInto<Color>>(self, value: V) -> Result<Self, <V>::Error> {
        let color = value.try_into()?;
        Ok(self.attribute(NodeAttribute::FontColor(color)))
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
                write!(w, "{}={}", key, &value)?;

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
    Label(Label),
    Shape(Option<Shape>),

    /// The color of the outline
    Color(Color),
    FillColor(Color),
    FontColor(Color),
    Unknown(String, String),
}

impl NodeAttribute {
    pub fn pair(&self) -> (&str, String) {
        match self {
            Self::Label(value) => ("label", value.as_string()),
            Self::Shape(value) => (
                "shape",
                match value {
                    Some(shape) => shape.as_str().to_string(),
                    None => String::from("none"),
                },
            ),
            Self::Color(color) => ("color", color.as_string()),
            Self::FillColor(color) => ("fillcolor", color.as_string()),
            Self::FontColor(color) => ("fontcolor", color.as_string()),
            Self::Unknown(key, value) => (key, sanitize(value)),
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
