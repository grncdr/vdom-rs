
#[macro_export]
macro_rules! vdom {
    ($tag:ident { $($attr:tt)* } [ $($child:tt)* ]) => {
        {
            let mut node = vdom!($tag);
            vdom!(@set_attrs node $($attr)*);
            vdom!(@add_children node $($child)*);
            node
        }
    };

    ($tag:ident { $($attr:tt)* }) => {
        {
            let mut node = vdom!($tag);
            vdom!(@set_attrs node $($attr)*);
            node
        }
    };

    ($tag:ident [$($child:tt)*]) => {
        {
            let mut node = vdom!($tag);
            vdom!(@add_children node $($child)*);
            node
        }
    };

    ($tag:ident) => {{
        use vdom::Node;
        Node::new(stringify!($tag))
    }};

    (@add_children $parent:ident text!( $text:expr ) $($rest:tt)*) => {
        $parent.append_string(($text).into());
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident format!( $($args:expr),* ) $($rest:tt)*) => {
        $parent.append_string(format!($( $args ),*));
        vdom!(@add_children $parent $($rest)*);
    };


    (@add_children $parent:ident $tag:ident { $($attr:tt)* } [ $($own_children:tt)* ] $($rest:tt)*) => {
        {
            let child = vdom!($tag { $($attr)* } [ $( $own_children )* ]);
            $parent.append_child(child);
        };
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident $tag:ident { $($attr:tt)* } $($rest:tt)*) => {
        {
            let child = vdom!($tag { $($attr)* });
            $parent.append_child(child);
        };
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident $tag:ident [ $($own_children:tt)* ] $($rest:tt)*) => {
        {
            let child = vdom!($tag [ $($own_children)* ]);
            $parent.append_child(child);
        };
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident $func_name:ident ( $( $args:expr ),* ) $($rest:tt)*) => {
        $parent.append_child($func_name($($args),*));
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident ( $elem:expr ) $($rest:tt)*) => {
        $parent.append_child($elem);
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident $tag:ident $($rest:tt)*) => {
        {
            let child = vdom!($tag);
            $parent.append_child(&child);
        };
        vdom!(@add_children $parent $($rest)*);
    };

    (@add_children $parent:ident) => {/* done */};

    (@set_attrs $node:ident on $ty:ty | $evt:ident | $body:expr ; $( $rest:tt )*) => {
        $node.add_event_listener(move |$evt : $ty| $body);
        vdom!(@set_attrs $node $($rest)*);
    };

    (@set_attrs $node:ident on $ty:ty | $evt:ident | $body:expr ) => {
        $node.add_event_listener(move |$evt : $ty| $body);
    };

    (@set_attrs $node:ident $name:ident = $value:expr ; $( $rest:tt )*) => {
        $node.add_attribute($name($value));
        vdom!(@set_attrs $node $($rest)*);
    };

    (@set_attrs $node:ident $name:ident = $value:expr ) => {
        $node.add_attribute($name($value));
    };

    (@set_attrs $node:ident) => {/* done */};
}


/// Generates a public newtype wrapper for an HTML attribute and an
/// implementation of super::Attribute.
///
/// Example:
///
/// ```rust
/// attr!(class, property, className);
///
/// vdom!(p { class = "Neat" })
/// ```rust
macro_rules! attr {
    ($rust_name:ident, $attr_fn:ident, $html_name:ident) => {
        /// Instantiate an `Attr` that will set the DOM elements `$html_name` $attr_fn.
        pub fn $rust_name<T: Into<::stdweb::Value>>(val: T) -> Attr {
            super::attribute::Attr::$attr_fn(stringify!($html_name), val)
        }
    }
}

/// Generates attribute factory functions.
macro_rules! attrs {
    [$(($rust_name:ident, $attr_fn:ident, $html_name:ident)),*] => {
        $(attr!($rust_name, $attr_fn, $html_name);)*
    }
}
