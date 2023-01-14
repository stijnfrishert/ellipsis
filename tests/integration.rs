use ellipsis::{export, Graph, Node};
use indoc::indoc;

#[test]
fn empty() {
    let graph = Graph::default();

    let mut vec = Vec::new();
    export(&graph, &mut vec).unwrap();

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
    export(&graph, &mut vec).unwrap();

    let expected = indoc! {"
        digraph {
          a [label=A, shape=box]
        }"
    };

    assert_eq!(String::from_utf8(vec).unwrap(), expected);
}
