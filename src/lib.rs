use std::{fs::File, io, path::Path, process::Command};
use tempfile::NamedTempFile;

mod edge;
mod graph;
mod node;

pub(crate) mod utils;

pub use edge::{Edge, EdgeAttribute};
pub use graph::Graph;
pub use node::{Node, NodeAttribute, Shape};

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

    pub fn render(&self, path: impl AsRef<Path>, write_dot_file: bool) -> io::Result<bool> {
        let path = path.as_ref();

        if write_dot_file {
            let dot_path = path.with_extension("dot");
            let dot_file = File::create(&dot_path)?;
            self.write(&dot_file)?;

            Self::render_from_file(&dot_path, path)
        } else {
            let temp = NamedTempFile::new().unwrap();
            self.write(&temp)?;

            Self::render_from_file(temp.path(), path)
        }
    }

    pub fn render_from_file(dot: impl AsRef<Path>, path: impl AsRef<Path>) -> io::Result<bool> {
        let status = Command::new("dot")
            .arg("-Tpng")
            .arg("-o")
            .arg(path.as_ref())
            .arg(dot.as_ref())
            .status()?;

        Ok(status.success())
    }
}
