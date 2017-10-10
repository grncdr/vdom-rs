use stdweb::web::{Node, INode};

pub struct DomIterator {
    root: Node,
    current: Node,
    finished: bool,
}

impl DomIterator {
    pub fn new(root: Node) -> Self {
        DomIterator {
            root: root.clone(),
            current: root,
            finished: false,
        }
    }
}

impl Iterator for DomIterator {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        if self.finished {
            return None;
        }
        // the node that will be returned
        let here = self.current.clone();

        match self.current.first_child() {
            Some(child) => {
                self.current = child;
            }
            None if self.current.as_ref() == self.root.as_ref() => {
                self.finished = true;
            }
            None => {
                match self.current.next_sibling() {
                    Some(sibling) => {
                        self.current = sibling;
                    }
                    None => {
                        let mut next = self.current.clone();
                        loop {
                            match next.parent_node() {
                                Some(parent) => {
                                    if parent.as_ref() == self.root.as_ref() {
                                        // we're done!
                                        self.finished = true;
                                        break;
                                    } else if let Some(aunt) = parent.next_sibling() {
                                        self.current = aunt;
                                        break;
                                    } else {
                                        next = parent;
                                    }
                                }
                                None => {
                                    // this indicates a bug, as we should always
                                    // encounter the root when traversing back up
                                    // the tree
                                    panic!("ran out of parent nodes in DomIterator");
                                }
                            }
                        }
                    }
                }
            }
        }

        return Some(here);
    }
}
