use ellipsis::{Dot, Edge, Graph, Node, Shape};
use indoc::indoc;

fn compare(dot: &Dot, expected: &str) {
    let written = dot.write_to_string().unwrap();

    assert_eq!(written, expected);
}

#[test]
fn empty() {
    let dot = Dot::new(false, Graph::new(Some("root".to_string())));

    let expected = indoc! {"
        graph root {
        }"
    };

    compare(&dot, expected);
}

#[test]
fn single_node() {
    let dot = Dot::new(
        false,
        Graph::new(None).node(Node::new("a").label("A").shape(Some(Shape::Box))),
    );

    let expected = indoc! {"
        graph {
          a [label=A, shape=box]
        }"
    };

    compare(&dot, expected);
}

#[test]
fn edge_undirected() {
    let dot = Dot::new(
        false,
        Graph::new(None).edge(Edge::new("a", "b").label("E").pen_width(2.0)),
    );

    let expected = indoc! {"
        graph {
          a -- b [label=E, penwidth=2]
        }"
    };

    compare(&dot, expected);
}

#[test]
fn edge_directed() {
    let dot = Dot::new(
        true,
        Graph::new(None).edge(Edge::new("a", "b").label("E").pen_width(2.0)),
    );

    let expected = indoc! {"
        digraph {
          a -> b [label=E, penwidth=2]
        }"
    };

    compare(&dot, expected);
}

#[test]
fn subgraph() {
    let dot = Dot::new(
        false,
        Graph::new(None).subgraph(Graph::new(None).node(Node::new("a"))),
    );

    let expected = indoc! {"
        graph {
          subgraph {
            a
          }
        }"
    };

    compare(&dot, expected);
}

#[test]
fn cluster() {
    let dot = Dot::new(
        false,
        Graph::new(None).subgraph(Graph::new(None).cluster().node(Node::new("a"))),
    );

    let expected = indoc! {"
        graph {
          subgraph cluster {
            a
          }
        }"
    };

    compare(&dot, expected);
}

#[test]
fn whitespace() {
    let dot = Dot::new(
        false,
        Graph::new(None)
            .subgraph(Graph::new(None))
            .node(Node::new("a"))
            .node(Node::new("b"))
            .edge(Edge::new("a", "b")),
    );

    let expected = indoc! {"
        graph {
          subgraph {
          }

          a
          b

          a -- b
        }"
    };

    compare(&dot, expected);
}
