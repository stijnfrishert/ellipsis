use crate::{
    utils::{sanitize, write_attribute, Attribute},
    Edge, Label, Node,
};
use indent_write::io::IndentWriter;
use std::io;

#[derive(Default)]
pub struct Graph {
    pub id: Option<String>,
    cluster: bool,
    pub attributes: Vec<GraphAttribute>,
    pub subgraphs: Vec<Graph>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(name: Option<String>) -> Self {
        Self {
            id: name,
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

    // --- Attributes --- //

    pub fn compound(self, compound: bool) -> Self {
        self.attribute(GraphAttribute::Compound(compound))
    }

    pub fn label(self, label: impl Into<Label>) -> Self {
        self.attribute(GraphAttribute::Label(label.into()))
    }

    pub fn margin(self, x: f32, y: f32) -> Self {
        self.attribute(GraphAttribute::Margin(x, y))
    }

    pub fn attribute(mut self, attribute: GraphAttribute) -> Self {
        self.attributes.push(attribute);
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

        if let Some(name) = &self.id {
            if cluster {
                let name = format!("cluster_{name}");
                write!(w, " {}", sanitize(&name))?;
            } else {
                write!(w, " {}", sanitize(name))?;
            };
        } else if cluster {
            write!(w, " cluster")?;
        }

        writeln!(w, " {{")?;

        let mut indented = IndentWriter::new("  ", &mut w);
        let mut indented: &mut dyn io::Write = &mut indented;

        let mut whitespace = false;

        // Subgraphs
        if !self.subgraphs.is_empty() {
            if whitespace {
                writeln!(indented)?;
            }

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

            whitespace = true;
        }

        // Nodes
        if !self.nodes.is_empty() {
            if whitespace {
                writeln!(indented)?;
            }

            for node in &self.nodes {
                node.write(&mut indented)?;
                writeln!(indented)?;
            }

            whitespace = true;
        }

        // Edges
        if !self.edges.is_empty() {
            if whitespace {
                writeln!(indented)?;
            }

            for edge in &self.edges {
                edge.write(directed, &mut indented)?;
                writeln!(indented)?;
            }

            whitespace = true;
        }

        // Attributes
        if !self.attributes.is_empty() {
            if whitespace {
                writeln!(indented)?;
            }

            for attribute in &self.attributes {
                write_attribute(attribute, &mut indented)?;
                writeln!(indented)?;
            }
        }

        write!(w, "}}")?;

        Ok(())
    }
}

pub(crate) enum GraphType {
    Root,
    Subgraph { cluster: bool },
}

pub enum GraphAttribute {
    Compound(bool),
    Label(Label),
    Margin(f32, f32),
}

impl Attribute for GraphAttribute {
    fn pair(&self) -> (&str, String) {
        match self {
            Self::Compound(compound) => (
                "compound",
                String::from(if *compound { "true" } else { "false" }),
            ),
            Self::Label(label) => ("label", label.as_string()),
            Self::Margin(x, y) => ("margin", format!("\"{x},{y}\"")),
        }
    }
}
