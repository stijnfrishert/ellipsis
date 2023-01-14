use std::io;

pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn write(&self, mut w: impl io::Write) -> io::Result<()> {
        write!(w, "{}={}", self.name, self.value)
    }
}
