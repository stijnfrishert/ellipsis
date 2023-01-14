use ellipsis::{write_to_string, Edge, Graph, Node, Shape};
use indoc::indoc;

fn compare(graph: &Graph, directed: bool, expected: &str) {
    let written = write_to_string(graph, directed).unwrap();

    assert_eq!(written, expected);
}

#[test]
fn empty() {
    let graph = Graph::new();

    let expected = indoc! {"
        graph {
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn single_node() {
    let graph = Graph::new().node(Node::new("a").label("A").shape(Some(Shape::Box)));

    let expected = indoc! {"
        graph {
          a [label=A, shape=box]
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn edge_undirected() {
    let graph = Graph::new().edge(Edge::new("a", "b").label("E").pen_width(2.0));

    let expected = indoc! {"
        graph {
          a -- b [label=E, penwidth=2]
        }"
    };

    compare(&graph, false, expected);
}

#[test]
fn edge_directed() {
    let graph = Graph::new().edge(Edge::new("a", "b").label("E").pen_width(2.0));

    let expected = indoc! {"
        digraph {
          a -> b [label=E, penwidth=2]
        }"
    };

    compare(&graph, true, expected);
}

#[test]
fn subgraph() {
    let graph = Graph::new().subgraph(Graph::new().node(Node::new("a")));

    let expected = indoc! {"
        graph {
          subgraph {
            a
          }
        }"
    };

    compare(&graph, false, expected);
}
