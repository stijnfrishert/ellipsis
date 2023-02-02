use std::{fs::File, io, path::Path, process::Command};
use tempfile::NamedTempFile;
use thiserror::Error;

mod color;
mod edge;
mod graph;
mod label;
mod node;

mod compass_point;
pub(crate) mod utils;

pub use color::{Color, ColorParseError};
pub use compass_point::CompassPoint;
pub use edge::{Edge, EdgeAttribute, EdgeStyle};
pub use graph::Graph;
pub use label::Label;
pub use node::{Node, NodeAttribute, NodeStyle, Shape};

pub struct Dot {
    directed: bool,
    graph: Graph,
}

impl Dot {
    pub fn new(directed: bool, graph: Graph) -> Self {
        Self { directed, graph }
    }

    pub fn write(&self, mut w: impl io::Write) -> io::Result<()> {
        self.graph
            .write(self.directed, graph::GraphType::Root, &mut w)
    }

    pub fn write_to_string(&self) -> io::Result<String> {
        let mut vec = Vec::new();
        self.write(&mut vec)?;
        Ok(String::from_utf8(vec).unwrap())
    }

    pub fn render(&self, path: impl AsRef<Path>, write_dot_file: bool) -> Result<(), RenderError> {
        let path = path.as_ref();

        if write_dot_file {
            let dot_path = path.with_extension("dot");
            let dot_file = File::create(&dot_path)?;
            self.write(&dot_file)?;

            Self::run_dot(&dot_path, path)?;
        } else {
            let temp = NamedTempFile::new().unwrap();
            self.write(&temp)?;

            Self::run_dot(temp.path(), path)?;
        }

        Ok(())
    }

    pub fn run_dot(dot: impl AsRef<Path>, path: impl AsRef<Path>) -> Result<(), RenderError> {
        let status = Command::new("dot")
            .arg("-Tpng")
            .arg("-o")
            .arg(path.as_ref())
            .arg(dot.as_ref())
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(RenderError::Dot(status.code()))
        }
    }
}

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("File I/O failed")]
    IO(#[from] io::Error),

    #[error("Running `dot` failed")]
    Dot(Option<i32>),
}
