use crate::{utils::sanitize, Color, Label};
use std::io;

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

    pub fn label(self, value: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::Label(value.into()))
    }

    pub fn color(self, value: Color) -> Self {
        self.attribute(EdgeAttribute::Color(value))
    }

    pub fn style(self, value: EdgeStyle) -> Self {
        self.attribute(EdgeAttribute::Style(value))
    }

    pub fn pen_width(self, value: f32) -> Self {
        self.attribute(EdgeAttribute::PenWidth(value))
    }

    pub fn head_label(self, value: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::HeadLabel(value.into()))
    }

    pub fn tail_label(self, value: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::TailLabel(value.into()))
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

pub enum EdgeAttribute {
    Label(Label),
    Color(Color),
    Style(EdgeStyle),
    PenWidth(f32),
    HeadLabel(Label),
    TailLabel(Label),
    Unknown(String, String),
}

impl EdgeAttribute {
    pub fn pair(&self) -> (&str, String) {
        match self {
            Self::Label(value) => ("label", value.as_string()),
            Self::Color(value) => ("color", value.as_string()),
            Self::Style(value) => ("style", value.as_str().to_string()),
            Self::PenWidth(value) => ("penwidth", format!("{value}")),
            Self::HeadLabel(value) => ("headlabel", value.as_string()),
            Self::TailLabel(value) => ("taillabel", value.as_string()),
            Self::Unknown(key, value) => (key, sanitize(value)),
        }
    }
}

pub enum EdgeStyle {
    Dashed,
    Dotted,
    Solid,
    Invisible,
    Bold,
    Tapered,
}

impl EdgeStyle {
    pub fn as_str(&self) -> &str {
        match self {
            EdgeStyle::Dashed => "dashed",
            EdgeStyle::Dotted => "dotted",
            EdgeStyle::Solid => "solid",
            EdgeStyle::Invisible => "invis",
            EdgeStyle::Bold => "bold",
            EdgeStyle::Tapered => "tapered",
        }
    }
}
