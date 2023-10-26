use super::untyped::*;

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_function(&mut self, function: &Function) -> T;
    fn visit_call(&mut self, call: &Call) -> T;
    fn visit_block(&mut self, block: &Vec<Expr>) -> T;
    fn visit_conditional(&mut self, conditional: &Conditional) -> T;
    fn visit_loop(&mut self, loop_: &Loop) -> T;
    fn visit_assignment(&mut self, assignment: &Assignment) -> T;
    fn visit_return(&mut self, return_: &Return) -> T;
    fn visit_identifier(&mut self, identifier: &Identifier) -> T;
    fn visit_literal(&mut self, literal: &LiteralKind) -> T;
    fn visit_parameter(&mut self, parameter: &Parameter) -> T;
    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) -> T;
    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr) -> T;
}
