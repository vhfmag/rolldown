use std::cell::Cell;

use oxc::{
  allocator::Box,
  ast::{ast, AstBuilder},
  span::{Atom, SourceType, SPAN},
};

use super::TakeIn;

impl<'ast> TakeIn<'ast> for ast::VariableDeclarationKind {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::Var
  }
}
impl<'ast> TakeIn<'ast> for ast::ThisExpression {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    builder.this_expression(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::VariableDeclaration<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      kind: TakeIn::dummy(builder),
      declarations: TakeIn::dummy(builder),
      declare: TakeIn::dummy(builder),
    }
  }
}
impl<'ast> TakeIn<'ast> for ast::Declaration<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::VariableDeclaration(Box::new_in(TakeIn::dummy(builder), alloc))
  }
}
impl<'ast> TakeIn<'ast> for ast::ExpressionStatement<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), expression: TakeIn::dummy(builder) }
  }
}
impl<'ast> TakeIn<'ast> for ast::FunctionType {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::FunctionDeclaration
  }
}

impl<'ast> TakeIn<'ast> for ast::FormalParameterKind {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::Signature
  }
}

impl<'ast> TakeIn<'ast> for ast::FormalParameters<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      kind: TakeIn::dummy(builder),
      items: TakeIn::dummy(builder),
      rest: TakeIn::dummy(builder),
    }
  }
}
impl<'ast> TakeIn<'ast> for ast::ClassBody<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), body: TakeIn::dummy(builder) }
  }
}
impl<'ast> TakeIn<'ast> for ast::ClassType {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::ClassDeclaration
  }
}
impl<'ast> TakeIn<'ast> for ast::Class<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      r#type: TakeIn::dummy(builder),
      span: TakeIn::dummy(builder),
      id: TakeIn::dummy(builder),
      super_class: TakeIn::dummy(builder),
      body: TakeIn::dummy(builder),
      type_parameters: TakeIn::dummy(builder),
      super_type_parameters: TakeIn::dummy(builder),
      implements: TakeIn::dummy(builder),
      decorators: TakeIn::dummy(builder),
      r#abstract: TakeIn::dummy(builder),
      declare: TakeIn::dummy(builder),
      scope_id: Cell::default(),
    }
  }
}
impl<'ast> TakeIn<'ast> for ast::Function<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      r#type: TakeIn::dummy(builder),
      span: TakeIn::dummy(builder),
      id: TakeIn::dummy(builder),
      generator: TakeIn::dummy(builder),
      r#async: TakeIn::dummy(builder),
      declare: TakeIn::dummy(builder),
      params: TakeIn::dummy(builder),
      body: TakeIn::dummy(builder),
      type_parameters: TakeIn::dummy(builder),
      return_type: TakeIn::dummy(builder),
      this_param: TakeIn::dummy(builder),
      scope_id: Cell::default(),
    }
  }
}
impl<'ast> TakeIn<'ast> for ast::Expression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::ThisExpression(Box::new_in(TakeIn::dummy(builder), alloc))
  }
}

impl<'ast> TakeIn<'ast> for ast::IdentifierName<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), name: TakeIn::dummy(builder) }
  }
}

impl<'ast> TakeIn<'ast> for ast::StaticMemberExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      object: TakeIn::dummy(builder),
      property: TakeIn::dummy(builder),
      optional: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::IdentifierReference<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      name: TakeIn::dummy(builder),
      reference_id: Cell::default(),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::Program<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      source_type: SourceType::default(),
      directives: TakeIn::dummy(builder),
      hashbang: TakeIn::dummy(builder),
      body: TakeIn::dummy(builder),
      scope_id: Cell::default(),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::VariableDeclarator<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      kind: TakeIn::dummy(builder),
      id: TakeIn::dummy(builder),
      init: TakeIn::dummy(builder),
      definite: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::BindingPattern<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      kind: TakeIn::dummy(builder),
      type_annotation: TakeIn::dummy(builder),
      optional: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::BindingPatternKind<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::BindingIdentifier(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::BindingIdentifier<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), name: TakeIn::dummy(builder), symbol_id: Cell::default() }
  }
}

impl<'ast> TakeIn<'ast> for Atom<'ast> {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Atom::from("")
  }
}

impl<'ast> TakeIn<'ast> for ast::CallExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      callee: TakeIn::dummy(builder),
      arguments: TakeIn::dummy(builder),
      optional: TakeIn::dummy(builder),
      type_parameters: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ArrowFunctionExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      expression: TakeIn::dummy(builder),
      r#async: TakeIn::dummy(builder),
      params: TakeIn::dummy(builder),
      body: TakeIn::dummy(builder),
      type_parameters: TakeIn::dummy(builder),
      return_type: TakeIn::dummy(builder),
      scope_id: Cell::default(),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::FunctionBody<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      directives: TakeIn::dummy(builder),
      statements: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::FormalParameter<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      pattern: TakeIn::dummy(builder),
      accessibility: TakeIn::dummy(builder),
      readonly: TakeIn::dummy(builder),
      decorators: TakeIn::dummy(builder),
      r#override: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::SequenceExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), expressions: TakeIn::dummy(builder) }
  }
}

impl<'ast> TakeIn<'ast> for ast::ParenthesizedExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), expression: TakeIn::dummy(builder) }
  }
}

impl<'ast> TakeIn<'ast> for ast::AssignmentExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      operator: TakeIn::dummy(builder),
      left: TakeIn::dummy(builder),
      right: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::AssignmentTarget<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::AssignmentTargetIdentifier(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::SimpleAssignmentTarget<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::AssignmentTargetIdentifier(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::ArrayAssignmentTarget<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      elements: TakeIn::dummy(builder),
      rest: TakeIn::dummy(builder),
      trailing_comma: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ObjectAssignmentTarget<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      properties: TakeIn::dummy(builder),
      rest: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::AssignmentTargetPropertyIdentifier<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      init: TakeIn::dummy(builder),
      span: TakeIn::dummy(builder),
      binding: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::AssignmentTargetMaybeDefault<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::AssignmentTargetIdentifier(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::AssignmentTargetPropertyProperty<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      name: TakeIn::dummy(builder),
      span: TakeIn::dummy(builder),
      binding: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::AssignmentTargetWithDefault<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      binding: TakeIn::dummy(builder),
      init: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ObjectExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      properties: TakeIn::dummy(builder),
      trailing_comma: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ObjectProperty<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      kind: TakeIn::dummy(builder),
      key: TakeIn::dummy(builder),
      value: TakeIn::dummy(builder),
      init: TakeIn::dummy(builder),
      method: TakeIn::dummy(builder),
      shorthand: TakeIn::dummy(builder),
      computed: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ObjectPropertyKind<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::ObjectProperty(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::PropertyKind {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::Init
  }
}

impl<'ast> TakeIn<'ast> for ast::PropertyKey<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self::Identifier(TakeIn::dummy(builder))
  }
}

impl<'ast> TakeIn<'ast> for ast::UnaryExpression<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      operator: TakeIn::dummy(builder),
      argument: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::StringLiteral<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), value: TakeIn::dummy(builder) }
  }
}
impl<'ast> TakeIn<'ast> for ast::ImportOrExportKind {
  fn dummy(_builder: AstBuilder<'ast>) -> Self {
    Self::Value
  }
}

impl<'ast> TakeIn<'ast> for ast::ImportDeclaration<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      specifiers: TakeIn::dummy(builder),
      source: TakeIn::dummy(builder),
      with_clause: TakeIn::dummy(builder),
      import_kind: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ObjectPattern<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: TakeIn::dummy(builder),
      properties: TakeIn::dummy(builder),
      rest: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::BindingProperty<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self {
      span: SPAN,
      key: TakeIn::dummy(builder),
      value: TakeIn::dummy(builder),
      shorthand: TakeIn::dummy(builder),
      computed: TakeIn::dummy(builder),
    }
  }
}

impl<'ast> TakeIn<'ast> for ast::ReturnStatement<'ast> {
  fn dummy(builder: AstBuilder<'ast>) -> Self {
    Self { span: TakeIn::dummy(builder), argument: TakeIn::dummy(builder) }
  }
}
