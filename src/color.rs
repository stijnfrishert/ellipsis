use hex::{FromHex, FromHexError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgba {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    },
    Hsv {
        hue: f32,
        saturation: f32,
        value: f32,
    },
    Name(String),
}

impl Color {
    pub fn as_string(&self) -> String {
        match self {
            Self::Rgba {
                red,
                green,
                blue,
                alpha,
            } => format!("#{red:02x}{green:02x}{blue:02x}{alpha:02x}"),
            Self::Hsv {
                hue,
                saturation,
                value,
            } => format!("{hue} {saturation} {value}"),
            Self::Name(name) => name.clone(),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ColorParseError {
    #[error("The string was empty")]
    Empty,

    #[error("The hex value was invalid")]
    InvalidRgba(#[from] FromHexError),

    #[error("Incorrect float represntations of HSV")]
    InvalidHsv,
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('#') if s.len() == 9 => {
                let hex = <[u8; 4]>::from_hex(&s[1..])?;
                Ok(Self::Rgba {
                    red: hex[0],
                    green: hex[1],
                    blue: hex[2],
                    alpha: hex[3],
                })
            }
            Some('#') if s.len() == 7 => {
                let hex = <[u8; 3]>::from_hex(&s[1..])?;
                Ok(Self::Rgba {
                    red: hex[0],
                    green: hex[1],
                    blue: hex[2],
                    alpha: 0xFF,
                })
            }
            Some(_) => {
                let words = s.split_whitespace().collect::<Vec<&str>>();
                if words.len() == 3 {
                    Ok(Self::Hsv {
                        hue: words[0]
                            .parse::<f32>()
                            .map_err(|_| ColorParseError::InvalidHsv)?,
                        saturation: words[1]
                            .parse::<f32>()
                            .map_err(|_| ColorParseError::InvalidHsv)?,
                        value: words[2]
                            .parse::<f32>()
                            .map_err(|_| ColorParseError::InvalidHsv)?,
                    })
                } else {
                    Err(ColorParseError::InvalidHsv)
                }
            }
            None => Err(ColorParseError::Empty),
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<Color>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_string() {
        assert_eq!(
            Color::Rgba {
                red: 0,
                green: 255,
                blue: 128,
                alpha: 30
            }
            .as_string(),
            "#00ff801e"
        );

        assert_eq!(
            Color::Hsv {
                hue: 0.5,
                saturation: 0.25,
                value: 0.8
            }
            .as_string(),
            "0.5 0.25 0.8"
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "#00ff801e".parse::<Color>(),
            Ok(Color::Rgba {
                red: 0,
                green: 255,
                blue: 128,
                alpha: 30
            })
        );

        assert_eq!(
            "#00ff80".parse::<Color>(),
            Ok(Color::Rgba {
                red: 0,
                green: 255,
                blue: 128,
                alpha: 255
            })
        );

        assert_eq!(
            "0.5 0.25 0.8".parse::<Color>(),
            Ok(Color::Hsv {
                hue: 0.5,
                saturation: 0.25,
                value: 0.8
            })
        );
    }
}
