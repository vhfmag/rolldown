use super::TakeIn;

use oxc::allocator::{Box, Vec};
use oxc::ast::AstBuilder;

impl<'ast, T: TakeIn<'ast>> TakeIn<'ast> for Box<'ast, T> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Box::new_in(TakeIn::dummy(builder), &builder.allocator)
  }
}

impl<'ast, T> TakeIn<'ast> for Vec<'ast, T> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Vec::new_in(&builder.allocator)
  }
}
