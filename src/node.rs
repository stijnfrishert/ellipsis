use crate::Attribute;
use std::io;

pub struct Node {
    pub id: String,
    pub attributes: Vec<Attribute>,
}

impl Node {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            attributes: Vec::new(),
        }
    }

    pub fn attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push(Attribute {
            name: name.into(),
            value: value.into(),
        });

        self
    }

    pub fn write(&self, mut w: impl io::Write) -> io::Result<()> {
        write!(w, "{}", self.id)?;

        if !self.attributes.is_empty() {
            write!(w, " [")?;

            let mut count = self.attributes.len();
            for attribute in &self.attributes {
                attribute.write(&mut w)?;

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
