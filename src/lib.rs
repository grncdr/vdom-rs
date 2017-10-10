#![feature(fnbox)]

#[macro_use]
extern crate stdweb;

#[macro_use]
mod macros;

mod node;
mod component;
mod events;
mod diff;
mod dom_iterator;
mod apply;
mod root;

#[cfg(test)]
mod diff_tests;

pub mod attribute;

pub use self::node::*;
pub use self::component::*;
pub use self::attribute::{Attribute, Attr};
pub use self::root::*;
