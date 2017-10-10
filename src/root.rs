use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::{document, Element, Node as DNode, INode};

use super::node::Node as VNode;
use super::component::*;

pub struct Root<Msg: 'static + Debug, C: Component<Msg>>(Rc<RootState<Msg, C>>);

impl<Msg, C> Root<Msg, C>
where
    Msg: 'static + Debug,
    C: Component<Msg>,
{
    pub fn send(&self, msg: Msg) {
        send(self.0.clone(), msg)
    }
}

struct RootState<Msg: 'static, C: Component<Msg>> {
    comp: RefCell<C>,
    dnode: RefCell<DNode>,
    vnode: RefCell<VNode<Msg>>,
}

pub fn render<M, C>(comp: C, target: Element) -> Root<M, C>
where
    M: 'static + Debug,
    C: 'static + Component<M>,
{
    let root = Rc::new(RootState {
        vnode: RefCell::new(VNode::new("div")),
        dnode: RefCell::new(target.as_node().clone()),
        comp: RefCell::new(comp),
    });
    redraw(root.clone());
    Root(root)
}


fn create_receiver<Msg, C>(root: Rc<RootState<Msg, C>>) -> Callback<Msg>
where
    Msg: 'static + Debug,
    C: 'static + Component<Msg>,
{
    Rc::new(move |msg| send(root.clone(), msg))
}

fn send<Msg, C>(root: Rc<RootState<Msg, C>>, msg: Msg)
where
    Msg: 'static + Debug,
    C: 'static + Component<Msg>,
{
    let recur = create_receiver(root.clone());
    {
        match root.comp.try_borrow_mut() {
            Ok(mut comp) => {
                println!("updating with message: {:?}", msg);
                comp.update(msg, recur)
            }
            Err(_) => {
                panic!(
                    "attempted to update while already updating, this is probably due to synchronously calling recur"
                )
            }
        }
    }
    redraw(root);
}

fn redraw<Msg, C>(root: Rc<RootState<Msg, C>>)
where
    Msg: Debug + 'static,
    C: 'static + Component<Msg>,
{
    use super::diff::diff;
    use super::apply::apply;

    let next_vnode = root.comp.borrow().view();
    let mut vnode = root.vnode.borrow_mut();
    {
        let patches = diff(&vnode, &next_vnode);
        println!("Patches: {:?}", patches);
        let mut dnode = root.dnode.borrow_mut();
        let send = create_receiver(root.clone());
        apply(&mut dnode, patches, send);
    }
    *vnode = next_vnode;
}

/// Create a new DOM element for the given `super::VNode`
pub fn create_element<Msg>(vnode: &VNode<Msg>, update: &Callback<Msg>) -> DNode
where
    Msg: Sized + Debug + 'static,
{
    let dnode = document().create_element(vnode.tag);

    for (_, attr) in vnode.attributes.iter() {
        attr.set(&dnode);
    }

    for child in vnode.children.iter() {
        use super::Child::*;

        match *child {
            Text(ref string) => {
                dnode.append_child(&document().create_text_node(string));
            }
            Node(ref child) => {
                dnode.append_child(&create_element(child, update));
            }
        }
    }

    for listener in vnode.listeners.iter() {
        listener.install(&dnode, update.clone());
    }

    dnode.as_node().clone()
}
