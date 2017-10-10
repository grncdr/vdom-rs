use std::fmt::Debug;
use super::{Node, Attr, Child};

#[derive(Debug, PartialEq)]
pub enum Operation<'node, Msg: 'static + Debug> {
    ReplaceNode(&'node Node<Msg>),
    ReplaceText(&'node str),
    RemoveAttribute(&'node Attr),
    SetAttribute(&'node Attr),
    RemoveLast(i32),
    Append(&'node [Child<Msg>]),
    Insert(i32, &'node Node<Msg>),
}

impl<'a, M: 'static + Debug> Operation<'a, M> {
    fn at(self, index: &i32) -> Patch<'a, i32, M> {
        Patch {
            node: *index,
            operation: self,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Patch<'node, K, M: 'static + Debug> {
    pub node: K,
    pub operation: Operation<'node, M>,
}

// transform the key type of this patch, used to associate patches with dom nodes
impl<'node, K, M: 'static + Debug> Patch<'node, K, M> {
    pub fn at<K2>(self, node: K2) -> Patch<'node, K2, M> {
        Patch {
            node: node,
            operation: self.operation,
        }
    }
}

pub fn diff<'root, M: 'static + Debug>(
    old: &'root Node<M>,
    new: &'root Node<M>,
) -> Vec<Patch<'root, i32, M>> {
    let mut patches: Vec<Patch<'root, i32, M>> = Vec::with_capacity(32);
    let mut index = 0i32;
    diff_node(old, new, &mut patches, &mut index);
    patches
}

fn diff_node<'root, 'node: 'root, M: 'static + Debug>(
    old: &'node Node<M>,
    new: &'node Node<M>,
    patches: &mut Vec<Patch<'root, i32, M>>,
    index: &mut i32,
) {
    use self::Operation::*;

    /* todo - investigate whether it's worth deriving PartialEq for Node
	if (old == new) {
		return;
	}
	*/

    // Bail if you run into different types of nodes. Implies that the
    // structure has changed significantly and it's not worth a diff.
    if new.tag != old.tag {
        patches.push(ReplaceNode(new).at(index));
        return;
    }

    diff_attributes(old, new, patches, index);
    diff_children(old, new, patches, index);
}

fn diff_attributes<'root, 'node: 'root, M: 'static + Debug>(
    old: &'node Node<M>,
    new: &'node Node<M>,
    patches: &mut Vec<Patch<'root, i32, M>>,
    index: &i32,
) {
    use std::collections::HashSet;
    use self::Operation::*;
    let keys: HashSet<_> = old.attributes.keys().chain(new.attributes.keys()).collect();
    for key in keys.into_iter() {
        if !new.attributes.contains_key(key) {
            patches.push(RemoveAttribute(old.attributes.get(key).unwrap()).at(index));
        } else if !old.attributes.contains_key(key) {
            patches.push(SetAttribute(new.attributes.get(key).unwrap()).at(index));
        } else {
            let new_attr = new.attributes.get(key);
            if old.attributes.get(key) != new_attr {
                patches.push(SetAttribute(new_attr.unwrap()).at(index));
            }
        }
    }
}

fn diff_children<'root, 'node: 'root, M: 'static + Debug>(
    old_parent: &'node Node<M>,
    new_parent: &'node Node<M>,
    patches: &mut Vec<Patch<'root, i32, M>>,
    index: &mut i32,
) {
    use self::Operation::*;
    let old_len = old_parent.children.len();
    let new_len = new_parent.children.len();

    // Figure out if there are inserts or removals

    if old_len > new_len {
        patches.push(RemoveLast((old_len - new_len) as i32).at(index));
    } else if old_len < new_len {
        patches.push(Append(&new_parent.children[old_len..]).at(index));
    }

    // Pairwise diff everything else
    let pairs = old_parent.children.iter().zip(new_parent.children.iter());

    for (old_child, new_child) in pairs {
        *index += 1;
        match (old_child, new_child) {
            (&Child::Node(ref old_node), &Child::Node(ref new_node)) => {
                diff_node(old_node, new_node, patches, index);
            }
            (&Child::Text(ref old_text), &Child::Text(ref new_text)) => {
                if old_text != new_text {
                    patches.push(ReplaceText(new_text.as_str()).at(index))
                }
            }
            (_, &Child::Text(ref new_text)) => {
                patches.push(ReplaceText(new_text.as_str()).at(index))
            }
            (_, &Child::Node(ref new_node)) => patches.push(ReplaceNode(new_node).at(index)),
        }
    }

    if old_len > new_len {
        // advance the node counter to compensate for the nodes we are removing from the dom.
        // this is needed to keep node indexes in sync with those generated in apply.
        *index += count_children(&old_parent.children[new_len..]);
    }
}

fn count_children<M>(children: &[Child<M>]) -> i32 {
    children.iter().fold(0, |count, child| {
        count + 1 +
            match child {
                &Child::Text(_) => 0,
                &Child::Node(ref node) => count_children(&node.children[..]),
            }
    })
}

////////////  KEYED DIFF  ////////////


/*
fn diff_keyed_children(old_parent, new_parent, patches, root_index) {
	let local_patches = Vec::new();

	let changes: HashMap<String, Entry> = HashMap::new(); // Dict String Entry
	let inserts: Vec<(i32, Entry)> = Vec::new(); // Array { index : Int, entry : Entry }
	// type Entry = { tag : String, vnode : VNode, index : Int, data : _ }

	let old_children = old_parent.children;
	let new_children = new_parent.children;
	let old_len = old_children.length;
	let new_len = new_children.length;
	let old_index = 0;
	let new_index = 0;

	let index = root_index;

	while (old_index < old_len && new_index < new_len)
	{
		let old = old_children[old_index];
		let new = new_children[new_index];

		let old_key = old._0;
		let new_key = new._0;
		let old_node = old._1;
		let new_node = new._1;

		// check if keys match

		if (old_key === new_key)
		{
			index++;
			diff_node(old_node, new_node, local_patches, index);
			index += old_node.descendants_count || 0;

			old_index++;
			new_index++;
			continue;
		}

		// look ahead 1 to detect insertions and removals.

		let old_look_ahead = old_index + 1 < old_len;
		let new_look_ahead = new_index + 1 < new_len;

		if (old_look_ahead)
		{
			let old_next = old_children[old_index + 1];
			let old_next_key = old_next._0;
			let old_next_node = old_next._1;
			let old_match = new_key === old_next_key;
		}

		if (new_look_ahead)
		{
			let new_next = new_children[new_index + 1];
			let new_next_key = new_next._0;
			let new_next_node = new_next._1;
			let new_match = old_key === new_next_key;
		}


		// swap old and new
		if (old_look_ahead && new_look_ahead && new_match && old_match)
		{
			index++;
			diff_node(old_node, new_next_node, local_patches, index);
			insert_node(changes, local_patches, old_key, new_node, new_index, inserts);
			index += old_node.descendants_count || 0;

			index++;
			remove_node(changes, local_patches, old_key, old_next_node, index);
			index += old_next_node.descendants_count || 0;

			old_index += 2;
			new_index += 2;
			continue;
		}

		// insert new
		if (new_look_ahead && new_match)
		{
			index++;
			insert_node(changes, local_patches, new_key, new_node, new_index, inserts);
			diff_node(old_node, new_next_node, local_patches, index);
			index += old_node.descendants_count || 0;

			old_index += 1;
			new_index += 2;
			continue;
		}

		// remove old
		if (old_look_ahead && old_match)
		{
			index++;
			remove_node(changes, local_patches, old_key, old_node, index);
			index += old_node.descendants_count || 0;

			index++;
			diff_node(old_next_node, new_node, local_patches, index);
			index += old_next_node.descendants_count || 0;

			old_index += 2;
			new_index += 1;
			continue;
		}

		// remove old, insert new
		if (old_look_ahead && new_look_ahead && old_next_key === new_next_key)
		{
			index++;
			remove_node(changes, local_patches, old_key, old_node, index);
			insert_node(changes, local_patches, new_key, new_node, new_index, inserts);
			index += old_node.descendants_count || 0;

			index++;
			diff_node(old_next_node, new_next_node, local_patches, index);
			index += old_next_node.descendants_count || 0;

			old_index += 2;
			new_index += 2;
			continue;
		}

		break;
	}

	// eat up any remaining nodes with remove_node and insert_node

	while (old_index < old_len)
	{
		index++;
		let old = old_children[old_index];
		let old_node = old._1;
		remove_node(changes, local_patches, old._0, old_node, index);
		index += old_node.descendants_count || 0;
		old_index++;
	}

	let end_inserts;
	while (new_index < new_len)
	{
		end_inserts = end_inserts || [];
		let new = new_children[new_index];
		insert_node(changes, local_patches, new._0, new._1, undefined, end_inserts);
		new_index++;
	}

	if (local_patches.length > 0 || inserts.length > 0 || typeof end_inserts !== 'undefined')
	{
		patches.push(make_patch('p-reorder', root_index, {
			patches: local_patches,
			inserts: inserts,
			end_inserts: end_inserts
		}));
	}
}


////////////  CHANGES FROM KEYED DIFF  ////////////

fn insert_node(changes, local_patches, key, vnode, new_index, inserts)
{
	let entry = changes[key];

	// never seen this key before
	if (typeof entry === 'undefined')
	{
		entry = {
			tag: 'insert',
			vnode: vnode,
			index: new_index,
			data: undefined
		};

		inserts.push({ index: new_index, entry: entry });
		changes[key] = entry;

		return;
	}

	// this key was removed earlier, old match!
	if (entry.tag === 'remove')
	{
		inserts.push({ index: new_index, entry: entry });

		entry.tag = 'move';
		let sub_patches = Vec::new();
		diff_node(entry.vnode, vnode, sub_patches, entry.index);
		entry.index = new_index;
		entry.data.data = {
			patches: sub_patches,
			entry: entry
		};

		return;
	}

	// this key has already been inserted or moved, old duplicate!
	insert_node(changes, local_patches, key + POSTFIX, vnode, new_index, inserts);
}


fn remove_node(changes, local_patches, key, vnode, index)
{
	let entry = changes[key];

	// never seen this key before
	if (typeof entry === 'undefined')
	{
		let patch = Patch(index, Remove)undefined);
		local_patches.push(patch);

		changes[key] = {
			tag: 'remove',
			vnode: vnode,
			index: index,
			data: patch
		};

		return;
	}

	// this key was inserted earlier, old match!
	if (entry.tag === 'insert')
	{
		entry.tag = 'move';
		let sub_patches = Vec::new();
		diff_node(vnode, entry.vnode, sub_patches, index);

		let patch = Patch(index, Remove){
			patches: sub_patches,
			entry: entry
		});
		local_patches.push(patch);

		return;
	}

	// this key has already been removed or moved, old duplicate!
	remove_node(changes, local_patches, key + POSTFIX, vnode, index);
}
*/
