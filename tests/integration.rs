use ellipsis::{write_to_string, Edge, Graph, Node, Shape};
use indoc::indoc;

fn compare(graph: &Graph, directed: bool, expected: &str) {
    let written = write_to_string(graph, directed).unwrap();

    assert_eq!(written, expected);
}

#[test]
fn empty() {
    let graph = Graph::new(Some("root".to_string()));

    let expected = indoc! {"
        graph root {
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn single_node() {
    let graph = Graph::new(None).node(Node::new("a").label("A").shape(Some(Shape::Box)));

    let expected = indoc! {"
        graph {
          a [label=A, shape=box]
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn edge_undirected() {
    let graph = Graph::new(None).edge(Edge::new("a", "b").label("E").pen_width(2.0));

    let expected = indoc! {"
        graph {
          a -- b [label=E, penwidth=2]
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn edge_directed() {
    let graph = Graph::new(None).edge(Edge::new("a", "b").label("E").pen_width(2.0));

    let expected = indoc! {"
        digraph {
          a -> b [label=E, penwidth=2]
        }"
    };

    compare(&graph, true, expected);
}

#[test]
fn subgraph() {
    let graph = Graph::new(None).subgraph(Graph::new(None).node(Node::new("a")));

    let expected = indoc! {"
        graph {
          subgraph {
            a
          }
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn cluster() {
    let graph = Graph::new(None).subgraph(Graph::new(None).cluster().node(Node::new("a")));

    let expected = indoc! {"
        graph {
          subgraph cluster {
            a
          }
        }"
    };

    compare(&graph, false, expected);
}
