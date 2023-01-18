pub enum CompassPoint {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center,
    Adjacent,
}

impl CompassPoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompassPoint::North => "n",
            CompassPoint::NorthEast => "ne",
            CompassPoint::East => "e",
            CompassPoint::SouthEast => "se",
            CompassPoint::South => "s",
            CompassPoint::SouthWest => "sw",
            CompassPoint::West => "w",
            CompassPoint::NorthWest => "nw",
            CompassPoint::Center => "c",
            CompassPoint::Adjacent => "_",
        }
    }
}
