use std::io;

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
}
