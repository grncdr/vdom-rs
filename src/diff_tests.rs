use super::diff::diff;
use super::diff::Operation::*;
use super::Node;


#[test]
fn test_diff_changed_tag() {
    let old_node: Node<()> = vdom!(
        a [ text!("some link") ]
    );

    let new_node = vdom!(
        div [ text!("some stuff") ]
    );

    assert_eq!(
        diff(&old_node, &new_node),
        vec![ ReplaceNode(&new_node).at(&0) ]
    );
}