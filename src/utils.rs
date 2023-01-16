use std::{io, iter::ExactSizeIterator};

pub fn sanitize(str: &str) -> String {
    let is_valid = match str.chars().next() {
        Some(x) if x.is_numeric() => str.parse::<f32>().is_ok(),
        Some(_) => str.chars().all(|c| c.is_alphanumeric() || c == '_'),
        None => return String::new(),
    };

    if is_valid {
        str.to_string()
    } else {
        format!("\"{str}\"")
    }
}

pub fn write_attributes<'a, I, A: 'a>(attributes: I, mut w: impl io::Write) -> io::Result<()>
where
    I: ExactSizeIterator<Item = &'a A>,
    A: Attribute,
{
    write!(w, " [")?;

    let mut count = attributes.len();
    for attribute in attributes {
        write_attribute(attribute, &mut w)?;

        count -= 1;
        if count > 0 {
            write!(w, ", ")?;
        }
    }
    write!(w, "]")?;

    Ok(())
}

pub fn write_attribute(attribute: &impl Attribute, mut w: impl io::Write) -> io::Result<()> {
    let (key, value) = attribute.pair();
    write!(w, "{}={}", key, &value)?;
    Ok(())
}

pub trait Attribute {
    fn pair(&self) -> (&str, String);
}
