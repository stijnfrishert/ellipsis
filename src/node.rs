use crate::{
    utils::{sanitize, write_attributes, Attribute},
    Color, Label,
};
use std::{fmt::Debug, io};

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

    pub fn color<C>(self, color: C) -> Self
    where
        C: TryInto<Color>,
        C::Error: Debug,
    {
        let color = color.try_into().unwrap();
        self.attribute(NodeAttribute::Color(color))
    }

    pub fn fill_color<C>(self, color: C) -> Self
    where
        C: TryInto<Color>,
        C::Error: Debug,
    {
        let color = color.try_into().unwrap();
        self.attribute(NodeAttribute::FillColor(color))
    }

    pub fn font_color<C>(self, color: C) -> Self
    where
        C: TryInto<Color>,
        C::Error: Debug,
    {
        let color = color.try_into().unwrap();
        self.attribute(NodeAttribute::FontColor(color))
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
            Self::Color(color) => ("color", sanitize(&color.as_string())),
            Self::FillColor(color) => ("fillcolor", sanitize(&color.as_string())),
            Self::FontColor(color) => ("fontcolor", sanitize(&color.as_string())),
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
    Egg,
    House,
    Hexagon,
    InvHouse,
    InvTrapezium,
    InvTriangle,
    Note,
    Octagon,
    Parallelogram,
    Pentagon,
    Plain,
    Polygon,
    Septagon,
    Square,
    Star,
    Trapezium,
    Triangle,
    Underline,
    Unknown(String),
}

impl Shape {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Box => "box",
            Self::Circle => "circle",
            Self::Diamond => "diamond",
            Self::Egg => "egg",
            Self::Hexagon => "hexagon",
            Self::House => "house",
            Self::InvHouse => "invhouse",
            Self::InvTrapezium => "invtrapezium",
            Self::InvTriangle => "invtriangle",
            Self::Note => "note",
            Self::Octagon => "octagon",
            Self::Pentagon => "pentagon",
            Self::Parallelogram => "parallelogram",
            Self::Plain => "plain",
            Self::Polygon => "polygon",
            Self::Septagon => "septagon",
            Self::Square => "square",
            Self::Star => "star",
            Self::Trapezium => "trapezium",
            Self::Triangle => "triangle",
            Self::Underline => "underline",
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
    pub fn as_str(&self) -> &'static str {
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
