use super::*;
/*
#[allow(dead_code)]
/// Returns the following tree.
///
/// ```text
/// root
/// |- child0
/// |- child1
///    |- child1-0
///    |- child1-1
/// ```
fn create_example_tree<'a>() -> PathedNode<&'a str> {
    PathedNode::create(
        "root",
        [
            PathedNode::create("child0", [].to_vec()),
            PathedNode::create(
                "child1",
                [
                    PathedNode::create("child1-0", [].to_vec()),
                    PathedNode::create("child1-1", [].to_vec()),
                ]
                .to_vec(),
            ),
        ]
        .to_vec(),
    )
}

#[test]
fn test_getting_child() {
    let root = create_example_tree();

    assert_eq!(root.get_child(0).unwrap().value, "child0");
    assert_eq!(root.get_node(Path([0])).unwrap().value, "child0");
}

#[test]
fn test_getting_descendant() {
    let root = create_example_tree();

    assert_eq!(
        root.get_child(1).unwrap().get_child(0).unwrap().value,
        "child1-0"
    );

    assert_eq!(root.get_node([1, 0]).unwrap().value, "child1-0");
}

#[test]
fn test_getting_none() {
    let root = create_example_tree();
    assert!(root.get_node(Path([100])).is_none())
}

#[test]
fn test_paths() {
    let root = create_example_tree();
    let path: Vec<_> = root.get_node([1, 0]).unwrap().path().collect();
    assert_eq!(path, [&1, &0].to_vec())
}
*/