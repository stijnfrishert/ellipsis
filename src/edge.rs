use crate::utils::sanitize;
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

    pub fn label(self, value: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::Label(value.into()))
    }

    pub fn pen_width(self, value: f32) -> Self {
        self.attribute(EdgeAttribute::PenWidth(value))
    }

    pub fn head_label(self, value: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::HeadLabel(value.into()))
    }

    pub fn tail_label(self, value: impl Into<String>) -> Self {
        self.attribute(EdgeAttribute::TailLabel(value.into()))
    }

    pub fn attribute(mut self, attribute: EdgeAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub(crate) fn write(&self, directed: bool, mut w: impl io::Write) -> io::Result<()> {
        if directed {
            write!(w, "{} -> {}", self.from, self.to)?;
        } else {
            write!(w, "{} -- {}", self.from, self.to)?;
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
    Label(String),
    PenWidth(f32),
    HeadLabel(String),
    TailLabel(String),
    Unknown(String, String),
}

impl EdgeAttribute {
    pub fn pair(&self) -> (&str, String) {
        match self {
            Self::Label(value) => ("label", value.clone()),
            Self::PenWidth(value) => ("penwidth", format!("{value}")),
            Self::HeadLabel(value) => ("headlabel", value.clone()),
            Self::TailLabel(value) => ("taillabel", value.clone()),
            Self::Unknown(key, value) => (key, value.clone()),
        }
    }
}
