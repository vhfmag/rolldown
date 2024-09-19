use oxc::{allocator::Allocator, syntax};

use super::TakeIn;

impl<'ast> TakeIn<'ast> for syntax::operator::UnaryOperator {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::Void
  }
}

impl<'ast> TakeIn<'ast> for syntax::operator::AssignmentOperator {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::Assign
  }
}
