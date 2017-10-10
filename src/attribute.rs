//! Strongly-typed abstractions for element attributes.

use std::fmt::Debug;
use stdweb::unstable::TryInto;
use stdweb::web::Element;
use stdweb::Value;

pub trait Attribute {
    fn key(&self) -> &'static str;
    fn set(&self, element: &Element);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AttrKind { Attribute, Property }

#[derive(Debug, PartialEq)]
pub struct Attr {
    kind: AttrKind,
    pub key: &'static str,
    pub value: Value,
}

impl Attr {
    pub fn property<T: Into<Value>>(key: &'static str, v: T) -> Self {
        Attr { kind: AttrKind::Property, key: key, value: v.into() }
    }

    pub fn attribute<T: Into<Value>>(key: &'static str, v: T) -> Self {
        Attr { kind: AttrKind::Attribute, key: key, value: v.into() }
    }

    pub fn set(&self, element: &Element) {
        match self.kind {
            AttrKind::Attribute => {
                js!( @{element}.setAttribute(@{self.key}, @{&self.value}); );
            }
            AttrKind::Property => {
                js!( @{element}[@{self.key}] = @{&self.value}; );
            }
        }
    }

    pub fn remove(&self, element: &Element) {
        match self.kind {
            AttrKind::Attribute => {
                js!( @{element}.removeAttribute(@{self.key}); );
            }
            AttrKind::Property => {
                js!( delete @{element}[@{self.key}]; );
            }
        }
    }
}

attrs! [
    // CSS
    (class, property, className),
    (style, property, style),

    // INPUTS
    (name, property, name),
    (type_, property, type),
    (value, property, value),
    (checked, property, checked),
    (disabled, property, disabled),

    //
    (href, property, href)
];

// TODO - CSS types/macros? ideally implement something like stylotron that
// transparently manages a sheet filled with atomic styles.

// TODO - copy the list of properties/attributes from Elm.