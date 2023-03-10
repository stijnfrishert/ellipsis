use crate::{
    utils::{sanitize, write_attributes, Attribute},
    Color, CompassPoint, Label,
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

    pub fn color<C>(self, color: C) -> Self
    where
        C: TryInto<Color>,
        C::Error: Debug,
    {
        let color = color.try_into().unwrap();
        self.attribute(EdgeAttribute::Color(color))
    }

    pub fn head_label(self, label: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::HeadLabel(label.into()))
    }

    pub fn head_port(self, port: impl Into<CompassPoint>) -> Self {
        self.attribute(EdgeAttribute::HeadPort(port.into()))
    }

    pub fn label(self, label: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::Label(label.into()))
    }

    pub fn lhead(self, lhead: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::LHead(lhead.into()))
    }

    pub fn ltail(self, ltail: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::LTail(ltail.into()))
    }

    pub fn style(self, style: EdgeStyle) -> Self {
        self.attribute(EdgeAttribute::Style(style))
    }

    pub fn tail_label(self, label: impl Into<Label>) -> Self {
        self.attribute(EdgeAttribute::TailLabel(label.into()))
    }

    pub fn tail_port(self, port: impl Into<CompassPoint>) -> Self {
        self.attribute(EdgeAttribute::TailPort(port.into()))
    }

    pub fn same_head(self, samehead: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::SameHead(samehead.into()))
    }

    pub fn same_tail(self, sametail: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::SameTail(sametail.into()))
    }

    pub fn pen_width(self, width: f32) -> Self {
        self.attribute(EdgeAttribute::PenWidth(width))
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
    HeadPort(CompassPoint),
    Label(Label),
    LHead(String),
    LTail(String),
    PenWidth(f32),
    SameHead(String),
    SameTail(String),
    Style(EdgeStyle),
    TailLabel(Label),
    TailPort(CompassPoint),
    Unknown(String, String),
}

impl Attribute for EdgeAttribute {
    fn pair(&self) -> (&str, String) {
        match self {
            Self::Color(color) => ("color", sanitize(&color.as_string())),
            Self::HeadLabel(label) => ("headlabel", label.as_string()),
            Self::HeadPort(compass_point) => ("headport", compass_point.as_str().to_string()),
            Self::Label(label) => ("label", label.as_string()),
            Self::LHead(head) => ("lhead", sanitize(head)),
            Self::LTail(tail) => ("ltail", sanitize(tail)),
            Self::PenWidth(width) => ("penwidth", format!("{width}")),
            Self::Style(style) => ("style", style.as_str().to_string()),
            Self::SameHead(samehead) => ("samehead", sanitize(samehead)),
            Self::SameTail(sametail) => ("sametail", sanitize(sametail)),
            Self::TailLabel(label) => ("taillabel", label.as_string()),
            Self::TailPort(compass_point) => ("tailport", compass_point.as_str().to_string()),
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
    pub fn as_str(&self) -> &'static str {
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
