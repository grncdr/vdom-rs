//! abstraction of stdweb::web::event
use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{Element, IEventTarget, EventListenerHandle};
use stdweb::web::event::ConcreteEvent;

pub trait VListener<Msg> {
    fn key(&self) -> &'static str;
    fn install(&self, element: &Element, update: Rc<Fn(Msg)>);
    fn remove(self);
}

pub struct ConcreteVListener<Evt: ConcreteEvent, Msg> {
    handle: RefCell<Option<EventListenerHandle>>,
    mapper: Rc<Fn(Evt) -> Msg>,
}

impl<Evt, Msg> ConcreteVListener<Evt, Msg>
where
    Evt: 'static + ConcreteEvent,
    Msg: 'static,
{
    pub fn new<F: 'static + Fn(Evt) -> Msg>(mapper: F) -> Self {
        ConcreteVListener {
            handle: RefCell::new(None),
            mapper: Rc::new(mapper),
        }
    }

    /*
    pub fn map<Tagger, TargetMsg>(self, tagger: Tagger) -> ConcreteVListener<Evt, TargetMsg>
    where
        TargetMsg: 'static,
        Tagger: 'static + Fn(Msg) -> TargetMsg,
    {
        let mapper = self.mapper;
        ConcreteVListener {
            handle: self.handle,
            mapper: Rc::new(move |evt| tagger(mapper(evt))),
        }
    }
    */
}

impl<Evt, Msg> VListener<Msg> for ConcreteVListener<Evt, Msg>
where
    Evt: ConcreteEvent + 'static,
    Msg: 'static,
{
    fn key(&self) -> &'static str {
        Evt::EVENT_TYPE
    }

    fn install(&self, element: &Element, update: Rc<Fn(Msg)>) {
        let mut handle = self.handle.borrow_mut();
        *handle = Some(element.add_event_listener({
            let update = update.clone();
            let map = self.mapper.clone();
            move |evt: Evt| { update(map(evt)); }
        }))
    }

    fn remove(self) {
        if let Some(handle) = self.handle.into_inner() {
            handle.remove();
        }
    }
}
