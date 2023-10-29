use crate::ast::typed;
use crate::ast::untyped;

impl typed::Expr {
    pub fn erase_type(&self) -> untyped::Expr {
        let kind = match &self.kind {
            typed::ExprKind::Identifier(ident) => {
                untyped::ExprKind::Identifier(untyped::Identifier {
                    name: ident.name.clone(),
                })
            }
            typed::ExprKind::Literal(lk) => {
                let k = match lk {
                    typed::LiteralKind::Int(i) => untyped::LiteralKind::Int(*i),
                    typed::LiteralKind::Bool(b) => untyped::LiteralKind::Bool(*b),
                };
                untyped::ExprKind::Literal(k)
            }
            typed::ExprKind::UnaryExpr(ue) => untyped::ExprKind::UnaryExpr(untyped::UnaryExpr {
                op: match ue.op {
                    typed::UnaryOp::Plus => untyped::UnaryOp::Plus,
                    typed::UnaryOp::Minus => untyped::UnaryOp::Minus,
                },
                child: Box::new(ue.child.erase_type()),
            }),
            typed::ExprKind::BinaryExpr(be) => untyped::ExprKind::BinaryExpr(untyped::BinaryExpr {
                op: match be.op {
                    typed::BinaryOp::Add => untyped::BinaryOp::Add,
                    typed::BinaryOp::Sub => untyped::BinaryOp::Sub,
                    typed::BinaryOp::Mul => untyped::BinaryOp::Mul,
                    typed::BinaryOp::LessThan => untyped::BinaryOp::LessThan,
                    typed::BinaryOp::GreaterThan => untyped::BinaryOp::GreaterThan,
                },
                lhs: Box::new(be.lhs.erase_type()),
                rhs: Box::new(be.rhs.erase_type()),
            }),
            typed::ExprKind::Return(ret) => untyped::ExprKind::Return(untyped::Return {
                value: Box::new(ret.value.erase_type()),
            }),
            typed::ExprKind::Assignment(asgn) => {
                untyped::ExprKind::Assignment(untyped::Assignment {
                    ident: untyped::Identifier {
                        name: asgn.ident.name.clone(),
                    },
                    value: Box::new(asgn.value.erase_type()),
                })
            }
            typed::ExprKind::Conditional(cond) => {
                untyped::ExprKind::Conditional(untyped::Conditional {
                    cond: Box::new(cond.cond.erase_type()),
                    on_true: Box::new(cond.on_true.erase_type()),
                    on_false: Box::new(cond.on_false.erase_type()),
                })
            }
            typed::ExprKind::Loop(loop_expr) => untyped::ExprKind::Loop(untyped::Loop {
                cond: Box::new(loop_expr.cond.erase_type()),
                body: Box::new(
                    loop_expr
                        .body
                        .iter()
                        .map(|expr| expr.erase_type())
                        .collect(),
                ),
            }),
            typed::ExprKind::Function(func) => untyped::ExprKind::Function(untyped::Function {
                ident: untyped::Identifier {
                    name: func.ident.name.clone(),
                },
                params: func
                    .params
                    .iter()
                    .map(|param| untyped::Parameter {
                        ident: untyped::Identifier {
                            name: param.ident.name.clone(),
                        },
                    })
                    .collect(),
                body: Box::new(func.body.iter().map(|expr| expr.erase_type()).collect()),
            }),
            typed::ExprKind::Call(call) => untyped::ExprKind::Call(untyped::Call {
                ident: untyped::Identifier {
                    name: call.ident.name.clone(),
                },
                args: call.args.iter().map(|expr| expr.erase_type()).collect(),
            }),
            typed::ExprKind::Block(block) => untyped::ExprKind::Block(untyped::Block {
                exprs: block.exprs.iter().map(|expr| expr.erase_type()).collect(),
            }),
        };

        untyped::Expr { kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::interpreter::{Interpreter, Value};
    use crate::ast::typed::{self, *};
    use crate::ast::untyped::{self, *};
    use crate::ast::visitor::Visitor;
    use crate::syntax::typed_parser::parse as parse_typed;
    use crate::syntax::untyped_parser::parse;

    #[test]
    fn test_type_erasure_and_interpretation() {
        let source = "
            def add(x: int, y: int) -> int { return x + y }
            add(3, 4)
        ";
        let typed_ast = parse_typed(source).unwrap();
        let untyped_ast: Vec<untyped::Expr> =
            typed_ast.iter().map(|expr| expr.erase_type()).collect();

        let mut interpreter = Interpreter::new();
        let _ = interpreter.visit_expr(&untyped_ast[0]); // Define the function
        let result = interpreter.visit_expr(&untyped_ast[1]); // Call the function
        assert_eq!(result, Some(Value::Int(7)));
    }

    #[test]
    fn test_type_erasure_and_interpretation_with_loops() {
        let source = "
            def add(x: int, y: int) -> int {
                result = 0
                while (x > 0) {
                    result = result + y
                    x = x - 1
                }
                return result
            }
            add(3, 4)
        ";
        let typed_ast = parse_typed(source).unwrap();
        let untyped_ast: Vec<untyped::Expr> =
            typed_ast.iter().map(|expr| expr.erase_type()).collect();

        let mut interpreter = Interpreter::new();
        let _ = interpreter.visit_expr(&untyped_ast[0]); // Define the function
        let result = interpreter.visit_expr(&untyped_ast[1]); // Call the function
        assert_eq!(result, Some(Value::Int(12)));
    }
}
