use crate::{Edge, Node};
use std::io;

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn write(&self, mut w: impl io::Write) -> io::Result<()> {
        writeln!(w, "digraph {{")?;

        for node in &self.nodes {
            write!(w, "  ")?;
            node.write(&mut w)?;
            writeln!(w)?;
        }

        if !self.nodes.is_empty() && !self.edges.is_empty() {
            writeln!(w)?;
        }

        for edge in &self.edges {
            write!(w, "  ")?;
            edge.write(true, &mut w)?;
            writeln!(w)?;
        }

        write!(w, "}}")?;

        Ok(())
    }
}
