use oxc::{ast::AstBuilder, span::Span};

mod impl_for_oxc_allocator;
mod impl_for_oxc_ast;
mod impl_for_oxc_syntax;

pub trait TakeIn<'ast> {
  /// Create a dummy value of `Self`
  fn dummy(builder: AstBuilder<'ast>) -> Self;

  /// A sugar function for using `std::mem::take` on `oxc_ast`.
  ///
  /// Replaces `self` with a [`TakeIn`] value of `Self`, returning the previous value stored in `self`.
  #[must_use]
  fn take_in(&mut self, builder: AstBuilder<'ast>) -> Self
  where
    Self: Sized,
  {
    std::mem::replace(self, Self::dummy(builder))
  }
}

macro_rules! impl_take_in_with_default_for {
  ($($t:ty),*) => {
    $(
      impl<'ast> TakeIn<'ast> for $t {
        fn dummy(_builder: AstBuilder<'ast>) -> Self {
          Default::default()
        }
      }
    )*
  };
}

impl_take_in_with_default_for!(Span);
impl_take_in_with_default_for!(bool);

impl<'ast, T> TakeIn<'ast> for Option<T> {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    None
  }
}
