use std::fmt::Debug;
use stdweb::web::{Node as DNode, INode};
use stdweb::unstable::TryInto;

use super::diff::Patch;
use super::root::create_element;
use super::component::Callback;
use super::dom_iterator::DomIterator;

pub fn apply<'node, M>(dnode: &mut DNode, patches: Vec<Patch<'node, i32, M>>, send: Callback<M>)
where
    M: 'static + Debug,
{
    if patches.len() == 0 {
        return;
    }

    let mut nodes = DomIterator::new(dnode.clone()).zip(0i32..);
    let (mut current_node, mut node_index) = nodes.next().unwrap();
    let mut with_nodes: Vec<Patch<'node, DNode, M>> = Vec::with_capacity(patches.len());

    for patch in patches.into_iter() {
        while node_index < patch.node {
            match nodes.next() {
                Some((next_node, next_index)) => {
                    current_node = next_node;
                    node_index = next_index;
                }
                None => panic!("Ran out of nodes before finishing all patches"),
            };
        }
        with_nodes.push(patch.at(current_node.clone()));
    }

    for patch in with_nodes.into_iter() {
        apply_patch(patch, &send);
    }
}

fn apply_patch<'node, Msg>(patch: Patch<'node, DNode, Msg>, send: &Callback<Msg>)
where
    Msg: 'static + Debug,
{
    use super::diff::Operation::*;

	#[cfg(debug)]
    js!(
		if (window.DEBUG_VDOM && /apply_patch/.test(window.DEBUG_VDOM.toString())) {
			console.log("Apply patch", @{&patch.node}, @{format!("{:?}", patch.operation)});
			debugger;
		}
	);

    match patch.operation {
        ReplaceNode(ref vnode) => {
            let new_dnode = create_element(vnode, send);
            patch.node.parent_node().unwrap().replace_child(
                &new_dnode,
                &patch.node,
            );
            //*dnode = new_dnode;
        }
        RemoveAttribute(ref attr) => {
            attr.remove(&patch.node.try_into().unwrap());
        }
        SetAttribute(ref attr) => {
            attr.set(&patch.node.try_into().unwrap());
        }
        ReplaceText(ref text) => {
            patch.node.set_text_content(text);
        }
        RemoveLast(ref count) => {
            js!(
				var i = @{*count};
				var domNode = @{patch.node.as_ref()};
				while (i--) {
					domNode.removeChild(domNode.lastChild);
				}
			);
        }
        Append(ref children) => {
            use super::Child;
            use stdweb::web::document;
            let doc = document();
            for child in children.iter() {
                match child {
                    &Child::Node(ref vnode) => {
                        patch.node.append_child(&create_element(vnode, send));
                    }
                    &Child::Text(ref text) => {
                        patch.node.append_child(&doc.create_text_node(text));
                    }
                }
            }
        }
        Insert(pos, ref vnode) => {
            let new_child = create_element(vnode, send);
            match patch
                .node
                .child_nodes()
                .into_iter()
                .skip(pos as usize)
                .next() {
                Some(sibling) => {
                    patch.node.insert_before(&sibling, &new_child);
                }
                None => {
                    patch.node.append_child(&new_child);
                }
            }
        }
    }
}
/*

////////////  APPLY FACTS  ////////////


function applyFacts(domNode, eventNode, facts)
{
	for (var key in facts)
	{
		var value = facts[key];

		switch (key)
		{
			case STYLE_KEY:
				applyStyles(domNode, value);
				break;

			case EVENT_KEY:
				applyEvents(domNode, eventNode, value);
				break;

			case ATTR_KEY:
				applyAttrs(domNode, value);
				break;

			case ATTR_NS_KEY:
				applyAttrsNS(domNode, value);
				break;

			case 'value':
				if (domNode[key] !== value)
				{
					domNode[key] = value;
				}
				break;

			default:
				domNode[key] = value;
				break;
		}
	}
}

*/
