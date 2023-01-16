use crate::{
    utils::{sanitize, write_attributes, Attribute},
    Color, Label,
};
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

    // --- Attributes --- //

    pub fn color<V: TryInto<Color>>(self, color: V) -> Result<Self, <V>::Error> {
        let color = color.try_into()?;
        Ok(self.attribute(NodeAttribute::Color(color)))
    }

    pub fn fill_color<V: TryInto<Color>>(self, color: V) -> Result<Self, <V>::Error> {
        let color = color.try_into()?;
        Ok(self.attribute(NodeAttribute::FillColor(color)))
    }

    pub fn font_color<V: TryInto<Color>>(self, color: V) -> Result<Self, <V>::Error> {
        let color = color.try_into()?;
        Ok(self.attribute(NodeAttribute::FontColor(color)))
    }

    pub fn label(self, label: impl Into<Label>) -> Self {
        self.attribute(NodeAttribute::Label(label.into()))
    }

    pub fn shape(self, shape: Option<Shape>) -> Self {
        self.attribute(NodeAttribute::Shape(shape))
    }

    pub fn style(self, style: NodeStyle) -> Self {
        self.attribute(NodeAttribute::Style(style))
    }

    pub fn attribute(mut self, attribute: NodeAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub(crate) fn write(&self, mut w: impl io::Write) -> io::Result<()> {
        write!(w, "{}", sanitize(&self.id))?;

        if !self.attributes.is_empty() {
            write_attributes(self.attributes.iter(), w)?;
        }

        Ok(())
    }
}

pub enum NodeAttribute {
    Color(Color),
    FillColor(Color),
    FontColor(Color),
    Label(Label),
    Shape(Option<Shape>),
    Style(NodeStyle),
    Unknown(String, String),
}

impl Attribute for NodeAttribute {
    fn pair(&self) -> (&str, String) {
        match self {
            Self::Color(color) => ("color", color.as_string()),
            Self::FillColor(color) => ("fillcolor", color.as_string()),
            Self::FontColor(color) => ("fontcolor", color.as_string()),
            Self::Label(label) => ("label", label.as_string()),
            Self::Shape(shape) => (
                "shape",
                match shape {
                    Some(shape) => shape.as_str().to_string(),
                    None => String::from("none"),
                },
            ),
            Self::Style(style) => ("style", style.as_str().to_string()),
            Self::Unknown(key, value) => (key, sanitize(value)),
        }
    }
}

pub enum Shape {
    Box,
    Circle,
    Diamond,
    Note,
    Square,
    Unknown(String),
}

impl Shape {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Box => "box",
            Self::Circle => "circle",
            Self::Diamond => "diamond",
            Self::Note => "note",
            Self::Square => "square",
            Self::Unknown(str) => str,
        }
    }
}

pub enum NodeStyle {
    Bold,
    Dashed,
    Diagonals,
    Dotted,
    Filled,
    Invisible,
    Rounded,
    Solid,
    Striped,
    Wedged,
}

impl NodeStyle {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bold => "bold",
            Self::Dashed => "dashed",
            Self::Diagonals => "diagonals",
            Self::Dotted => "dotted",
            Self::Filled => "filled",
            Self::Invisible => "invis",
            Self::Rounded => "rounded",
            Self::Solid => "solid",
            Self::Striped => "striped",
            Self::Wedged => "wedged",
        }
    }
}
