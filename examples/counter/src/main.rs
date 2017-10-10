use stdweb::web::event::ClickEvent;
use vdom::{Component, Node, Callback};

#[derive(Debug, Default)]
pub struct Counter(i32);

#[derive(Debug)]
pub enum CounterMsg { Inc, Dec }

impl Component<CounterMsg> for Counter {
    fn view(&self) -> Node<CounterMsg> {
        use stdweb::web::event::ClickEvent;
        vdom!(div [
            p [ format!("Counter value: {}", self.0) ]
            button { on ClickEvent |_evt| CounterMsg::Inc; }
                   [ text!("increment") ]
            button { on ClickEvent |_evt| CounterMsg::Dec; }
                   [ text!("decrement") ]
        ])
    }

    fn update(&mut self, msg: CounterMsg, _recur: Callback<CounterMsg>) {
        match msg {
            CounterMsg::Inc => self.0 += 1,
            CounterMsg::Dec => self.0 -= 1,
        }
    }
}