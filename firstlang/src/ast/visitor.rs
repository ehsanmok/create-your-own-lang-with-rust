use super::typed;
use super::untyped;

pub(crate) trait Input {}
impl Input for typed::Expr {}
impl Input for typed::Block {}
impl Input for typed::Function {}
impl Input for typed::Call {}
impl Input for typed::Conditional {}
impl Input for typed::Loop {}
impl Input for typed::Assignment {}
impl Input for typed::Return {}
impl Input for typed::Identifier {}
impl Input for typed::LiteralKind {}
impl Input for typed::Parameter {}
impl Input for typed::UnaryExpr {}
impl Input for typed::BinaryExpr {}

impl Input for untyped::Expr {}
impl Input for untyped::Block {}
impl Input for untyped::Function {}
impl Input for untyped::Call {}
impl Input for untyped::Conditional {}
impl Input for untyped::Loop {}
impl Input for untyped::Assignment {}
impl Input for untyped::Return {}
impl Input for untyped::Identifier {}
impl Input for untyped::LiteralKind {}
impl Input for untyped::Parameter {}
impl Input for untyped::UnaryExpr {}
impl Input for untyped::BinaryExpr {}

pub(crate) trait Visitor<In: Input> {
    type Output;
    fn visit_expr(&mut self, expr: &In) -> Self::Output;
    fn visit_function(&mut self, function: &In) -> Self::Output;
    fn visit_call(&mut self, call: &In) -> Self::Output;
    fn visit_block(&mut self, block: &In) -> Self::Output;
    fn visit_conditional(&mut self, conditional: &In) -> Self::Output;
    fn visit_loop(&mut self, loop_: &In) -> Self::Output;
    fn visit_assignment(&mut self, assignment: &In) -> Self::Output;
    fn visit_return(&mut self, return_: &In) -> Self::Output;
    fn visit_identifier(&mut self, identifier: &In) -> Self::Output;
    fn visit_literal(&mut self, literal: &In) -> Self::Output;
    fn visit_parameter(&mut self, parameter: &In) -> Self::Output;
    fn visit_unary_expr(&mut self, unary_expr: &In) -> Self::Output;
    fn visit_binary_expr(&mut self, binary_expr: &In) -> Self::Output;
}
