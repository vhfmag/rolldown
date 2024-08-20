use oxc::{
  allocator::{self, Allocator, Box, IntoIn},
  ast::{
    ast::{
      self, Argument, BindingIdentifier, BindingRestElement, ImportOrExportKind, Statement,
      TSThisParameter, TSTypeAnnotation, TSTypeParameterDeclaration, TSTypeParameterInstantiation,
    },
    AstBuilder,
  },
  span::{Atom, CompactStr, Span, SPAN},
  syntax::operator::UnaryOperator,
};

use crate::allocator_helpers::take_in::TakeIn;

type PassedStr<'a> = &'a str;

// `AstBuilder` is more suitable name, but it's already used in oxc.
pub struct AstSnippet<'ast> {
  pub builder: AstBuilder<'ast>,
}

impl<'ast> AstSnippet<'ast> {
  pub fn new(alloc: &'ast Allocator) -> Self {
    Self { builder: AstBuilder::new(alloc) }
  }

  #[inline]
  pub fn alloc(&self) -> &'ast Allocator {
    self.builder.allocator
  }

  pub fn atom(&self, value: &str) -> Atom<'ast> {
    self.builder.atom(value)
  }

  #[inline]
  pub fn id(&self, name: PassedStr, span: Span) -> ast::BindingIdentifier<'ast> {
    self.builder.binding_identifier(span, name)
  }

  #[inline]
  pub fn id_ref(&self, name: PassedStr, span: Span) -> ast::IdentifierReference<'ast> {
    self.builder.identifier_reference(span, name)
  }

  #[inline]
  pub fn id_name(&self, name: PassedStr, span: Span) -> ast::IdentifierName<'ast> {
    self.builder.identifier_name(span, name)
  }

  #[inline]
  pub fn id_ref_expr(&self, name: PassedStr, span: Span) -> ast::Expression<'ast> {
    self.builder.expression_identifier_reference(span, name)
  }

  pub fn member_expr_or_ident_ref(
    &self,
    object: ast::Expression<'ast>,
    names: &[CompactStr],
    span: Span,
  ) -> ast::Expression<'ast> {
    match names {
      [] => object,
      _ => ast::Expression::StaticMemberExpression(self.builder.alloc_static_member_expression(
        span,
        self.member_expr_or_ident_ref(object, &names[0..names.len() - 1], span),
        self.id_name(names[names.len() - 1].as_str(), span),
        false,
      )),
    }
  }

  /// `[object].[property]`
  pub fn literal_prop_access_member_expr(
    &self,
    object: PassedStr,
    property: PassedStr,
  ) -> ast::MemberExpression<'ast> {
    ast::MemberExpression::StaticMemberExpression(self.builder.alloc_static_member_expression(
      SPAN,
      self.id_ref_expr(object, SPAN),
      self.builder.identifier_name(SPAN, property),
      false,
    ))
  }

  /// `[object].[property]`
  #[inline]
  pub fn literal_prop_access_member_expr_expr(
    &self,
    object: PassedStr,
    property: PassedStr,
  ) -> ast::Expression<'ast> {
    ast::Expression::from(self.literal_prop_access_member_expr(object, property))
  }

  /// `name()`
  pub fn call_expr(&self, name: PassedStr) -> ast::CallExpression<'ast> {
    ast::CallExpression {
      callee: self.id_ref_expr(name, SPAN),
      arguments: self.builder.vec(),
      ..TakeIn::dummy(self.alloc())
    }
  }

  /// `name()`
  pub fn call_expr_expr(&self, name: PassedStr) -> ast::Expression<'ast> {
    ast::Expression::CallExpression(self.call_expr(name).into_in(self.alloc()))
  }

  /// `name(arg)`
  pub fn call_expr_with_arg_expr(&self, name: PassedStr, arg: PassedStr) -> ast::Expression<'ast> {
    let arg = ast::Argument::Identifier(self.id_ref(arg, SPAN).into_in(self.alloc()));
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
  }

  /// `name(arg)`
  pub fn call_expr_with_arg_expr_expr(
    &self,
    name: PassedStr,
    arg: ast::Expression<'ast>,
  ) -> ast::Expression<'ast> {
    let arg = ast::Argument::from(arg);
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
  }

  /// `name(arg1, arg2)`
  pub fn call_expr_with_2arg_expr(
    &self,
    name: PassedStr,
    arg1: PassedStr,
    arg2: PassedStr,
  ) -> ast::Expression<'ast> {
    let arg1 = ast::Argument::Identifier(self.builder.alloc_identifier_reference(SPAN, arg1));
    let arg2 = ast::Argument::Identifier(self.builder.alloc_identifier_reference(SPAN, arg2));
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg1);
    call_expr.arguments.push(arg2);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
  }

  /// `name(arg1, arg2)`
  pub fn call_expr_with_2arg_expr_expr(
    &self,
    name: PassedStr,
    arg1: ast::Expression<'ast>,
    arg2: ast::Expression<'ast>,
  ) -> ast::Expression<'ast> {
    let arg1 = ast::Argument::from(arg1);
    let arg2 = ast::Argument::from(arg2);
    let mut call_expr = self.call_expr(name);
    call_expr.arguments.push(arg1);
    call_expr.arguments.push(arg2);
    ast::Expression::CallExpression(call_expr.into_in(self.alloc()))
  }

  /// `name()`
  #[inline]
  pub fn call_expr_stmt(&self, name: PassedStr) -> ast::Statement<'ast> {
    ast::Statement::ExpressionStatement(
      self.builder.alloc_expression_statement(SPAN, self.call_expr_expr(name)),
    )
  }

  /// `var [name] = [init]`
  #[inline]
  pub fn var_decl_stmt(
    &self,
    name: PassedStr,
    init: ast::Expression<'ast>,
  ) -> ast::Statement<'ast> {
    ast::Statement::from(self.decl_var_decl(name, init))
  }

  /// `var [name] = [init]`
  pub fn decl_var_decl(
    &self,
    name: PassedStr,
    init: ast::Expression<'ast>,
  ) -> ast::Declaration<'ast> {
    let declarations = self.builder.vec1(self.builder.variable_declarator(
      SPAN,
      ast::VariableDeclarationKind::Var,
      self.builder.binding_pattern(
        self.builder.binding_pattern_kind_binding_identifier(SPAN, name),
        None::<TSTypeAnnotation>,
        false,
      ),
      Some(init),
      false,
    ));

    ast::Declaration::VariableDeclaration(self.builder.alloc_variable_declaration(
      SPAN,
      ast::VariableDeclarationKind::Var,
      declarations,
      false,
    ))
  }

  /// `var [name] = [init]`
  pub fn var_decl(
    &self,
    name: PassedStr,
    init: ast::Expression<'ast>,
  ) -> Box<'ast, ast::VariableDeclaration<'ast>> {
    let declarations = self.builder.vec1(self.builder.variable_declarator(
      SPAN,
      ast::VariableDeclarationKind::Var,
      self.builder.binding_pattern(
        self.builder.binding_pattern_kind_binding_identifier(SPAN, name),
        None::<TSTypeAnnotation>,
        false,
      ),
      Some(init),
      false,
    ));
    self.builder.alloc_variable_declaration(
      SPAN,
      ast::VariableDeclarationKind::Var,
      declarations,
      false,
    )
  }

  pub fn var_decl_multiple_names(
    &self,
    names: &[(&str, &str)],
    init: ast::Expression<'ast>,
  ) -> Box<'ast, ast::VariableDeclaration<'ast>> {
    let mut declarations = self.builder.vec_with_capacity(1);
    let mut properties = self.builder.vec();
    names.iter().for_each(|(imported, local)| {
      properties.push(self.builder.binding_property(
        SPAN,
        self.builder.property_key_from_identifier_name(self.id_name(imported, SPAN)),
        self.builder.binding_pattern(
          self.builder.binding_pattern_kind_binding_identifier(SPAN, *local),
          None::<TSTypeAnnotation>,
          false,
        ),
        false,
        false,
      ));
    });
    declarations.push(ast::VariableDeclarator {
      id: ast::BindingPattern {
        kind: ast::BindingPatternKind::ObjectPattern(
          ast::ObjectPattern { properties, ..TakeIn::dummy(self.alloc()) }.into_in(self.alloc()),
        ),
        ..TakeIn::dummy(self.alloc())
      },
      init: Some(init),
      ..TakeIn::dummy(self.alloc())
    });
    self.builder.alloc_variable_declaration(
      SPAN,
      ast::VariableDeclarationKind::Var,
      declarations,
      false,
    )
  }

  /// ```js
  ///  var require_foo = __commonJS((exports, module) => {
  ///    ...
  ///  });
  /// ```
  pub fn commonjs_wrapper_stmt(
    &self,
    binding_name: PassedStr,
    commonjs_name: PassedStr,
    statements: allocator::Vec<'ast, Statement<'ast>>,
  ) -> ast::Statement<'ast> {
    // (exports, module) => {}

    let mut arrow_expr = self.builder.alloc_arrow_function_expression(
      SPAN,
      false,
      false,
      None::<TSTypeParameterDeclaration>,
      self.builder.formal_parameters(
        SPAN,
        ast::FormalParameterKind::Signature,
        self.builder.vec_with_capacity(2),
        None::<BindingRestElement>,
      ),
      None::<TSTypeAnnotation>,
      self.builder.function_body(SPAN, self.builder.vec(), statements),
    );
    arrow_expr.params.items.push(self.builder.formal_parameter(
      SPAN,
      self.builder.vec(),
      self.builder.binding_pattern(
        self.builder.binding_pattern_kind_binding_identifier(SPAN, "exports"),
        None::<TSTypeAnnotation>,
        false,
      ),
      None,
      false,
      false,
    ));

    arrow_expr.params.items.push(self.builder.formal_parameter(
      SPAN,
      self.builder.vec(),
      self.builder.binding_pattern(
        self.builder.binding_pattern_kind_binding_identifier(SPAN, "module"),
        None::<TSTypeAnnotation>,
        false,
      ),
      None,
      false,
      false,
    ));

    //  __commonJS(...)
    let mut commonjs_call_expr = self.call_expr(commonjs_name);
    commonjs_call_expr.arguments.push(ast::Argument::ArrowFunctionExpression(arrow_expr));

    // var require_foo = ...

    let var_decl_stmt = self.var_decl_stmt(
      binding_name,
      ast::Expression::CallExpression(commonjs_call_expr.into_in(self.alloc())),
    );

    var_decl_stmt
  }

  /// ```js
  /// var init_foo = __esm(() => { ... });
  /// ```
  pub fn esm_wrapper_stmt(
    &self,
    binding_name: PassedStr,
    esm_fn_name: PassedStr,
    statements: allocator::Vec<'ast, Statement<'ast>>,
  ) -> ast::Statement<'ast> {
    // () => { ... }

    let arrow_expr = self.builder.alloc_arrow_function_expression(
      SPAN,
      false,
      false,
      None::<TSTypeParameterDeclaration>,
      self.builder.formal_parameters(
        SPAN,
        ast::FormalParameterKind::Signature,
        self.builder.vec(),
        None::<BindingRestElement>,
      ),
      None::<TSTypeAnnotation>,
      self.builder.function_body(SPAN, self.builder.vec(), statements),
    );

    //  __esm(...)
    let mut commonjs_call_expr = self.call_expr(esm_fn_name);
    commonjs_call_expr.arguments.push(ast::Argument::ArrowFunctionExpression(arrow_expr));

    // var init_foo = ...

    self.var_decl_stmt(
      binding_name,
      ast::Expression::CallExpression(commonjs_call_expr.into_in(self.alloc())),
    )
  }

  /// ```js
  /// (a, b)
  /// ```
  pub fn seq2_in_paren_expr(
    &self,
    a: ast::Expression<'ast>,
    b: ast::Expression<'ast>,
  ) -> ast::Expression<'ast> {
    let mut expressions = self.builder.vec_with_capacity(2);
    expressions.push(a);
    expressions.push(b);
    let seq_expr = ast::Expression::SequenceExpression(
      self.builder.alloc_sequence_expression(SPAN, expressions),
    );
    ast::Expression::ParenthesizedExpression(
      self.builder.alloc_parenthesized_expression(SPAN, seq_expr),
    )
  }

  pub fn number_expr(&self, value: f64, raw: &'ast str) -> ast::Expression<'ast> {
    ast::Expression::NumericLiteral(self.builder.alloc_numeric_literal(
      SPAN,
      value,
      raw,
      oxc::syntax::number::NumberBase::Decimal,
    ))
  }

  /// ```js
  ///  id = ...
  /// ￣￣ AssignmentTarget
  /// ```
  pub fn simple_id_assignment_target(
    &self,
    id: PassedStr,
    span: Span,
  ) -> ast::AssignmentTarget<'ast> {
    ast::AssignmentTarget::AssignmentTargetIdentifier(self.id_ref(id, span).into_in(self.alloc()))
  }

  /// ```js
  /// () => xx
  /// ```
  pub fn only_return_arrow_expr(&self, expr: ast::Expression<'ast>) -> ast::Expression<'ast> {
    let statements = self.builder.vec1(ast::Statement::ExpressionStatement(
      self.builder.alloc_expression_statement(SPAN, expr),
    ));
    ast::Expression::ArrowFunctionExpression(self.builder.alloc_arrow_function_expression(
      SPAN,
      true,
      false,
      None::<TSTypeParameterDeclaration>,
      self.builder.formal_parameters(
        SPAN,
        ast::FormalParameterKind::Signature,
        self.builder.vec(),
        None::<BindingRestElement>,
      ),
      None::<TSTypeAnnotation>,
      self.builder.function_body(SPAN, self.builder.vec(), statements),
    ))
  }

  /// `undefined` is acting like identifier, it might be shadowed by user code.
  pub fn void_zero(&self) -> ast::Expression<'ast> {
    ast::Expression::UnaryExpression(self.builder.alloc_unary_expression(
      SPAN,
      UnaryOperator::Void,
      self.number_expr(0.0, "0"),
    ))
  }

  pub fn string_literal(&self, value: PassedStr, span: Span) -> ast::StringLiteral<'ast> {
    ast::StringLiteral { span, value: self.atom(value) }
  }

  pub fn string_literal_expr(&self, value: PassedStr, span: Span) -> ast::Expression<'ast> {
    ast::Expression::StringLiteral(self.string_literal(value, span).into_in(self.alloc()))
  }

  pub fn import_star_stmt(&self, source: PassedStr, as_name: PassedStr) -> ast::Statement<'ast> {
    let specifiers = self.builder.vec1(ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(
      self.builder.alloc_import_namespace_specifier(SPAN, self.id(as_name, SPAN)),
    ));
    ast::Statement::ImportDeclaration(self.builder.alloc_import_declaration(
      SPAN,
      Some(specifiers),
      self.string_literal(source, SPAN),
      None,
      ImportOrExportKind::Value,
    ))
  }

  pub fn app_static_import_star_call_stmt(
    &self,
    as_name: &str,
    importee_source: &str,
  ) -> ast::Statement<'ast> {
    let mut declarations = allocator::Vec::new_in(self.alloc());

    let mut call_expr = self.call_expr("__static_import");
    call_expr.arguments.push(ast::Argument::StringLiteral(
      self.string_literal(importee_source, SPAN).into_in(self.alloc()),
    ));
    declarations.push(self.builder.variable_declarator(
      SPAN,
      ast::VariableDeclarationKind::Var,
      self.builder.binding_pattern(
        self.builder.binding_pattern_kind_binding_identifier(SPAN, as_name),
        None::<TSTypeAnnotation>,
        false,
      ),
      Some(ast::Expression::CallExpression(call_expr.into_in(self.alloc()))),
      false,
    ));

    ast::Statement::VariableDeclaration(self.builder.alloc_variable_declaration(
      SPAN,
      ast::VariableDeclarationKind::Var,
      declarations,
      false,
    ))
  }

  pub fn app_static_import_call_multiple_specifiers_stmt(
    &self,
    names: &[(&str, &str)],
    importee_source: &str,
  ) -> ast::Statement<'ast> {
    let mut declarations = self.builder.vec();
    let mut properties = self.builder.vec();
    names.iter().for_each(|(imported, local)| {
      properties.push(self.builder.binding_property(
        SPAN,
        self.builder.property_key_from_identifier_name(self.id_name(imported, SPAN)),
        self.builder.binding_pattern(
          self.builder.binding_pattern_kind_binding_identifier(SPAN, *local),
          None::<TSTypeAnnotation>,
          false,
        ),
        false,
        false,
      ));
    });
    let mut call_expr = self.call_expr("__static_import");
    call_expr.arguments.push(ast::Argument::StringLiteral(
      self.string_literal(importee_source, SPAN).into_in(self.alloc()),
    ));
    declarations.push(self.builder.variable_declarator(
      SPAN,
      ast::VariableDeclarationKind::Var,
      self.builder.binding_pattern(
        self.builder.binding_pattern_kind_object_pattern(
          SPAN,
          properties,
          None::<BindingRestElement>,
        ),
        None::<TSTypeAnnotation>,
        false,
      ),
      Some(ast::Expression::CallExpression(call_expr.into_in(self.alloc()))),
      false,
    ));

    ast::Statement::VariableDeclaration(self.builder.alloc_variable_declaration(
      SPAN,
      ast::VariableDeclarationKind::Var,
      declarations,
      false,
    ))
  }

  /// Promise.resolve().then(function() {})
  pub fn promise_resolve_then_call_expr(
    &self,
    span: Span,
    statements: allocator::Vec<'ast, Statement<'ast>>,
  ) -> ast::Expression<'ast> {
    let arguments = self.builder.vec1(Argument::FunctionExpression(self.builder.alloc_function(
      ast::FunctionType::FunctionExpression,
      SPAN,
      None::<BindingIdentifier>,
      false,
      false,
      false,
      None::<TSTypeParameterDeclaration>,
      None::<TSThisParameter>,
      self.builder.formal_parameters(
        SPAN,
        ast::FormalParameterKind::Signature,
        self.builder.vec_with_capacity(2),
        None::<BindingRestElement>,
      ),
      None::<TSTypeAnnotation>,
      Some(self.builder.function_body(SPAN, self.builder.vec(), statements)),
    )));

    let callee =
      ast::Expression::StaticMemberExpression(self.builder.alloc_static_member_expression(
        SPAN,
        ast::Expression::CallExpression(self.builder.alloc_call_expression(
          SPAN,
          ast::Expression::StaticMemberExpression(self.builder.alloc_static_member_expression(
            SPAN,
            self.id_ref_expr("Promise", SPAN),
            self.id_name("resolve", SPAN),
            false,
          )),
          None::<TSTypeParameterInstantiation>,
          self.builder.vec(),
          false,
        )),
        self.id_name("then", SPAN),
        false,
      ));
    ast::Expression::CallExpression(self.builder.alloc_call_expression(
      span,
      callee,
      None::<TSTypeParameterInstantiation>,
      arguments,
      false,
    ))
  }

  // return xxx
  pub fn return_stmt(&self, argument: ast::Expression<'ast>) -> ast::Statement<'ast> {
    ast::Statement::ReturnStatement(
      ast::ReturnStatement { argument: Some(argument), ..TakeIn::dummy(self.alloc()) }
        .into_in(self.alloc()),
    )
  }
}
