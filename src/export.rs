use crate::Graph;
use std::io;

pub fn export(graph: &Graph, mut w: impl io::Write) -> io::Result<()> {
    writeln!(w, "digraph {{")?;

    for node in &graph.nodes {
        write!(w, "  ")?;
        node.write(&mut w)?;
        writeln!(w)?;
    }

    write!(w, "}}")?;

    Ok(())
}
