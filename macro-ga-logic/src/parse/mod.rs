pub mod element;
pub mod expr;
pub mod lambda;
pub mod mvtype;

use std::iter::Peekable;

pub type Tokens = Peekable<proc_macro2::token_stream::IntoIter>;
