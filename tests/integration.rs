use ellipsis::{Edge, Graph, Node};
use indoc::indoc;

#[test]
fn empty() {
    let graph = Graph::default();

    let mut vec = Vec::new();
    graph.write(&mut vec).unwrap();

    let expected = indoc! {"
        digraph {
        }"
    };

    assert_eq!(String::from_utf8(vec).unwrap(), expected);
}

#[test]
fn single_node() {
    let mut graph = Graph::default();
    graph.nodes.push(
        Node::new("a")
            .attribute("label", "A")
            .attribute("shape", "box"),
    );

    let mut vec = Vec::new();
    graph.write(&mut vec).unwrap();

    let expected = indoc! {"
        digraph {
          a [label=A, shape=box]
        }"
    };

    assert_eq!(String::from_utf8(vec).unwrap(), expected);
}

#[test]
fn edge() {
    let mut graph = Graph::default();
    graph.edges.push(
        Edge::new("a", "b")
            .attribute("label", "E")
            .attribute("penwidth", "2"),
    );

    let mut vec = Vec::new();
    graph.write(&mut vec).unwrap();

    let expected = indoc! {"
        digraph {
          a -> b [label=E, penwidth=2]
        }"
    };

    assert_eq!(String::from_utf8(vec).unwrap(), expected);
}
