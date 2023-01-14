use std::io;

mod attribute;
mod edge;
mod graph;
mod node;

pub use attribute::Attribute;
pub use edge::Edge;
pub use graph::Graph;
pub use node::Node;

pub fn write(graph: &Graph, directed: bool, mut w: impl io::Write) -> io::Result<()> {
    graph.write(directed, false, &mut w)
}

pub fn write_to_string(graph: &Graph, directed: bool) -> io::Result<String> {
    let mut vec = Vec::new();
    write(graph, directed, &mut vec)?;
    Ok(String::from_utf8(vec).unwrap())
}
