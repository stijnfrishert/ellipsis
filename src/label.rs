use crate::utils::sanitize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    Text(String),
    HtmlLike(String),
}

impl Label {
    pub fn as_string(&self) -> String {
        match self {
            Self::Text(string) => sanitize(&string),
            Self::HtmlLike(string) => string.clone(),
        }
    }
}

impl<T> From<T> for Label
where
    T: Into<String>,
{
    fn from(str: T) -> Self {
        Label::Text(str.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label() {
        // Regular without quotes
        assert_eq!(Label::Text("a".into()).as_string(), "a");

        // Regular with quotes
        assert_eq!(Label::Text("a b".into()).as_string(), "\"a b\"");

        // HTML-like
        assert_eq!(
            Label::HtmlLike("<<bold>a</bold>>".into()).as_string(),
            "<<bold>a</bold>>"
        );
    }
}
