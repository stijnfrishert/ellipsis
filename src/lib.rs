use std::io;

mod edge;
mod graph;
mod node;

pub(crate) mod utils;

pub use edge::{Edge, EdgeAttribute};
pub use graph::Graph;
pub use node::{Node, NodeAttribute, Shape};

pub fn write(graph: &Graph, directed: bool, mut w: impl io::Write) -> io::Result<()> {
    graph.write(directed, graph::GraphType::Root, &mut w)
}

pub fn write_to_string(graph: &Graph, directed: bool) -> io::Result<String> {
    let mut vec = Vec::new();
    write(graph, directed, &mut vec)?;
    Ok(String::from_utf8(vec).unwrap())
}
