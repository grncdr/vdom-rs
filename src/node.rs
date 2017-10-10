use std::rc::Rc;
use std::collections::HashMap;
use std::boxed::FnBox;
use std::iter::FromIterator;

use stdweb::web::{Element, EventListenerHandle};
use stdweb::web::event::ConcreteEvent;

use super::attribute::Attr;
use super::events::{VListener, ConcreteVListener};

/// shit's complicated (TODO - document how this ended up being the way)
pub type ListenerInstaller<Msg> = FnBox(&Element, Rc<Fn(Msg)>) -> EventListenerHandle;


pub struct Node<Msg: 'static> {
    pub tag: &'static str,
    pub attributes: HashMap<&'static str, Attr>,
    pub children: Vec<Child<Msg>>,
    pub listeners: Vec<Box<VListener<Msg>>>,
}

impl<Msg> Node<Msg> {
    pub fn new(tag: &'static str) -> Self {
        Node {
            tag: tag,
            attributes: HashMap::new(),
            children: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn wrap_in<T, I>(tag: &'static str, things: I) -> Self
    where
        T: Into<Node<Msg>>,
        I: IntoIterator<Item = T>,
    {
        Node {
            tag: tag,
            attributes: HashMap::new(),
            children: things.into_iter().map(|x| Child::Node(x.into())).collect(),
            listeners: Vec::new(),
        }
    }

    pub fn add_event_listener<T, F>(&mut self, listener: F)
    where
        T: ConcreteEvent + 'static,
        F: Fn(T) -> Msg + 'static,
    {
        self.listeners.push(
            Box::new(ConcreteVListener::new(listener)),
        );
    }

    pub fn append_child(&mut self, node: Self) {
        self.children.push(Child::Node(node));
    }

    pub fn append_string(&mut self, text: String) {
        self.children.push(Child::Text(text))
    }

    pub fn add_attribute(&mut self, attribute: Attr) {
        self.attributes.insert(attribute.key, attribute);
    }
}

pub enum Child<Msg: 'static> {
    Text(String),
    Node(Node<Msg>),
}

impl<M> PartialEq for Child<M> {
    fn eq(&self, other: &Self) -> bool {
        use self::Child::*;

        match (self, other) {
            (&Text(ref s_string), &Text(ref o_string)) => s_string == o_string,
            (&Node(ref s_node), &Node(ref o_node)) => s_node == o_node,
            _ => false
        }
    }
}

use std::fmt::{Debug, Formatter, Result as FmtResult};

impl<M> PartialEq for Node<M> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag &&
        self.attributes == other.attributes &&
        self.children.len() == other.children.len() &&
        {
            for (self_child, other_child) in self.children.iter().zip(other.children.iter()) {
                if self_child != other_child {
                    return false;
                }
            }
            true
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<M> Debug for Node<M> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Node{{ tag: {:?}, children: {:?} }}",
            self.tag,
            self.children
        )
    }
}

impl<M> Debug for Child<M> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Child::Text(ref text) => write!(f, "{:?}", text),
            Child::Node(ref node) => write!(f, "{:?}", node),
        }
    }
}

impl<M> FromIterator<Node<M>> for Node<M> {
    fn from_iter<T: IntoIterator<Item = Node<M>>>(iter: T) -> Self {
        Node {
            tag: "div",
            attributes: HashMap::new(),
            children: iter.into_iter().map(Child::Node).collect(),
            listeners: Vec::new(),
        }
    }
}

impl<M> From<Node<M>> for Child<M> {
    fn from(node: Node<M>) -> Self {
        Child::Node(node)
    }
}

impl<M, T> From<T> for Child<M>
where
    T: Into<String>,
{
    fn from(text: T) -> Self {
        Child::Text(text.into())
    }
}

/*
impl<M> From<(usize, Node<M>)> for Child<M> {
    fn from(node: Node<M>) -> Self {
        Child::KeyedNode(usize, node)
    }
}
*/
