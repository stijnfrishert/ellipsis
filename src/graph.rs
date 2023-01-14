use crate::{utils::sanitize, Edge, Node};
use indent_write::io::IndentWriter;
use std::io;

#[derive(Default)]
pub struct Graph {
    pub name: Option<String>,
    cluster: bool,
    pub subgraphs: Vec<Graph>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn subgraph(mut self, subgraph: Graph) -> Self {
        self.subgraphs.push(subgraph);
        self
    }

    pub fn cluster(mut self) -> Self {
        self.cluster = true;
        self
    }

    pub fn node(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn edge(mut self, edge: Edge) -> Self {
        self.edges.push(edge);
        self
    }

    pub(crate) fn write(
        &self,
        directed: bool,
        graph_type: GraphType,
        mut w: impl io::Write,
    ) -> io::Result<()> {
        let cluster = match graph_type {
            GraphType::Root => {
                if directed {
                    write!(w, "digraph")?
                } else {
                    write!(w, "graph")?
                }

                false
            }
            GraphType::Subgraph { cluster } => {
                write!(w, "subgraph")?;
                cluster
            }
        };

        if let Some(name) = &self.name {
            write!(
                w,
                " {}{}",
                if cluster { "cluster_" } else { "" },
                sanitize(name)
            )?;
        } else if cluster {
            write!(w, " cluster")?;
        }

        writeln!(w, " {{")?;

        let mut indented = IndentWriter::new("  ", &mut w);
        let mut indented: &mut dyn io::Write = &mut indented;

        // Subgraphs
        for subgraph in &self.subgraphs {
            subgraph.write(
                directed,
                GraphType::Subgraph {
                    cluster: subgraph.cluster,
                },
                &mut indented,
            )?;
            writeln!(indented)?;
        }

        // Nodes
        for node in &self.nodes {
            node.write(&mut indented)?;
            writeln!(indented)?;
        }

        // Edges
        for edge in &self.edges {
            edge.write(directed, &mut indented)?;
            writeln!(indented)?;
        }

        write!(w, "}}")?;

        Ok(())
    }
}

pub enum GraphType {
    Root,
    Subgraph { cluster: bool },
}
