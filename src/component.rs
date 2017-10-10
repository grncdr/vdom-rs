use std::rc::Rc;
use vdom::node::Node;

pub type Callback<T> = Rc<Fn( T )>;

pub trait Component<Msg: 'static>: Sized + 'static {
    fn view(&self) -> Node<Msg>;
    fn update(&mut self, Msg, Callback<Msg>);
}