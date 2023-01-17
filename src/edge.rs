use crate::{
    utils::{sanitize, write_attributes, Attribute},
    Color, Label,
};
use std::{fmt::Debug, io};

pub struct Edge {
    pub from: String,
    pub to: String,
    pub attributes: Vec<EdgeAttribute>,
}

impl Edge {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            attributes: Vec::new(),
        }
    }

    // --- Attributes --- //

    pub fn label(self, label: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::Label(label.into()))
    }

    pub fn color<C>(self, color: C) -> Self
    where
        C: TryInto<Color>,
        C::Error: Debug,
    {
        let color = color.try_into().unwrap();
        self.attribute(EdgeAttribute::Color(color))
    }

    pub fn style(self, style: EdgeStyle) -> Self {
        self.attribute(EdgeAttribute::Style(style))
    }

    pub fn pen_width(self, width: f32) -> Self {
        self.attribute(EdgeAttribute::PenWidth(width))
    }

    pub fn head_label(self, label: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::HeadLabel(label.into()))
    }

    pub fn tail_label(self, label: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::TailLabel(label.into()))
    }

    pub fn attribute(mut self, attribute: EdgeAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub(crate) fn write(&self, directed: bool, mut w: impl io::Write) -> io::Result<()> {
        if directed {
            write!(w, "{} -> {}", sanitize(&self.from), sanitize(&self.to))?;
        } else {
            write!(w, "{} -- {}", sanitize(&self.from), sanitize(&self.to))?;
        }

        if !self.attributes.is_empty() {
            write_attributes(self.attributes.iter(), w)?;
        }

        Ok(())
    }
}

pub enum EdgeAttribute {
    Color(Color),
    HeadLabel(Label),
    Label(Label),
    PenWidth(f32),
    Style(EdgeStyle),
    TailLabel(Label),
    Unknown(String, String),
}

impl Attribute for EdgeAttribute {
    fn pair(&self) -> (&str, String) {
        match self {
            Self::Color(color) => ("color", sanitize(&color.as_string())),
            Self::HeadLabel(label) => ("headlabel", label.as_string()),
            Self::Label(label) => ("label", label.as_string()),
            Self::PenWidth(width) => ("penwidth", format!("{width}")),
            Self::Style(style) => ("style", style.as_str().to_string()),
            Self::TailLabel(label) => ("taillabel", label.as_string()),
            Self::Unknown(key, value) => (key, sanitize(value)),
        }
    }
}

pub enum EdgeStyle {
    Bold,
    Dashed,
    Dotted,
    Invisible,
    Solid,
    Tapered,
}

impl EdgeStyle {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Solid => "solid",
            Self::Invisible => "invis",
            Self::Bold => "bold",
            Self::Tapered => "tapered",
        }
    }
}
