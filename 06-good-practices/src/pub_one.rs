//! Documentation for thi module

use crate::private_two::foo;

/// `bar` calls [`foo`].
///
/// FYI: You can link to the documentation of other items in your crate (and dependencies) by
/// putting their name in brakets, like above.
pub fn bar() {
    foo();
}

/// This is a public struct in this module. It has exactly one public field: `a`.
#[derive(Debug)]
pub struct S {
    /// A public field, this can be set and get from everyone who has access to an instance of this
    /// struct.
    pub a: u32,
    /// This field is private and can only be accessed inside this module. And from no where else.
    b: u32,
}

/// Be careful with enums: In public enums every variant with every field is public. If you want to
/// prevent, that a field in your enum is set without a constructor, define an extra struct for it
/// and wrap it like in the `PrivateBS` variant.
pub enum E {
    /// Since everything is public, we need to document it
    S {
        /// and every field
        a: u32,
        /// also this
        b: u32,
    },
    /// But no tuple struct fields.
    PrivateBS(S),
}
