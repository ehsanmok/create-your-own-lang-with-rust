use crate::ast::untyped::{self, *};
use crate::ast::visitor::Visitor;

use derive_new::new;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Function(Function),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enviroment {
    pub values: HashMap<String, Value>,
}

impl Enviroment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interpreter {
    pub env: Enviroment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(),
        }
    }
}

impl Visitor<untyped::Expr> for Interpreter {
    type Output = Option<Value>;
    fn visit_expr(&mut self, expr: &Expr) -> Option<Value> {
        match &expr.kind {
            ExprKind::Literal(_) => self.visit_literal(expr),
            ExprKind::UnaryExpr(_) => self.visit_unary_expr(expr),
            ExprKind::BinaryExpr(_) => self.visit_binary_expr(expr),
            ExprKind::Conditional(_) => self.visit_conditional(expr),
            ExprKind::Function(_) => self.visit_function(expr),
            ExprKind::Call(_) => self.visit_call(expr),
            ExprKind::Block(_) => self.visit_block(expr),
            ExprKind::Identifier(_) => self.visit_identifier(expr),
            ExprKind::Return(_) => self.visit_return(expr),
            ExprKind::Assignment(_) => self.visit_assignment(expr),
            ExprKind::Loop(_) => self.visit_loop(expr),
        }
    }

    fn visit_literal(&mut self, literal: &untyped::Expr) -> Option<Value> {
        if let untyped::ExprKind::Literal(lk) = &literal.kind {
            match lk {
                LiteralKind::Int(i) => Some(Value::Int(*i)),
                LiteralKind::Bool(b) => Some(Value::Bool(*b)),
            }
        } else {
            panic!("Expected LiteralKind, found something else");
        }
    }

    fn visit_unary_expr(&mut self, unary_expr: &untyped::Expr) -> Option<Value> {
        if let untyped::ExprKind::UnaryExpr(ue) = &unary_expr.kind {
            let operand = self.visit_expr(&ue.child);
            match ue.op {
                UnaryOp::Plus => operand,
                UnaryOp::Minus => match operand {
                    Some(Value::Int(i)) => Some(Value::Int(-i)),
                    _ => panic!("Unary minus applied to non-integer value"),
                },
            }
        } else {
            panic!("Expected UnaryExpr, found something else");
        }
    }

    fn visit_binary_expr(&mut self, binary_expr: &untyped::Expr) -> Option<Value> {
        if let untyped::ExprKind::BinaryExpr(be) = &binary_expr.kind {
            let left = self.visit_expr(&be.lhs);
            let right = self.visit_expr(&be.rhs);
            match be.op {
                BinaryOp::Add => match (left, right) {
                    (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Int(l + r)),
                    _ => panic!("Addition applied to non-integer values"),
                },
                BinaryOp::Sub => match (left, right) {
                    (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Int(l - r)),
                    _ => panic!("Subtraction applied to non-integer values"),
                },
                BinaryOp::Mul => match (left, right) {
                    (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Int(l * r)),
                    _ => panic!("Multiplication applied to non-integer values"),
                },
                BinaryOp::LessThan => match (left, right) {
                    (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Bool(l < r)),
                    _ => panic!("Less-than applied to non-integer values"),
                },
                BinaryOp::GreaterThan => match (left, right) {
                    (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Bool(l > r)),
                    _ => panic!("Greater-than applied to non-integer values"),
                },
            }
        } else {
            panic!("Expected BinaryExpr, found something else");
        }
    }

    fn visit_conditional(&mut self, conditional: &untyped::Expr) -> Option<Value> {
        if let untyped::ExprKind::Conditional(conditional) = &conditional.kind {
            let cond = self.visit_expr(&conditional.cond);
            match cond {
                Some(Value::Bool(b)) => {
                    if b {
                        self.visit_expr(&conditional.on_true)
                    } else {
                        self.visit_expr(&conditional.on_false)
                    }
                }
                _ => panic!("Conditional applied to non-boolean value"),
            }
        } else {
            panic!("Expected Conditional, found something else");
        }
    }

    fn visit_block(&mut self, block: &untyped::Expr) -> Option<Value> {
        let block = if let untyped::ExprKind::Block(block) = &block.kind {
            block
        } else {
            panic!("Expected Block, found something else");
        };
        let mut return_value = None;
        for expr in block.exprs.iter() {
            return_value = self.visit_expr(expr);
        }
        return_value
    }

    fn visit_function(&mut self, func: &untyped::Expr) -> Option<Value> {
        let func = if let untyped::ExprKind::Function(func) = &func.kind {
            self.env
                .insert(func.ident.name.clone(), Value::Function(func.clone()));
            return None;
        } else {
            panic!("Expected Function, found something else");
        };
    }

    fn visit_call(&mut self, call: &untyped::Expr) -> Option<Value> {
        let call = if let untyped::ExprKind::Call(call) = &call.kind {
            call
        } else {
            panic!("Expected Call, found something else");
        };
        let (func_params, func_body) =
            if let Some(Value::Function(func)) = self.env.get(&call.ident.name) {
                (func.params.clone(), func.body.clone())
            } else {
                panic!("Called a non-function value or undefined function");
            };

        // Now, execute the function.
        for (param, arg) in func_params.iter().zip(call.args.iter()) {
            let arg_val = self.visit_expr(arg);
            self.env
                .insert(param.ident.to_string(), arg_val.clone().unwrap());
        }

        // Execute the function's body.
        // This is for a single expression body.
        // let return_value = self.visit_expr(&func_body);
        // return_value
        // Now support a block
        // Execute the function's body.
        // need to wrap the block in an Expr
        let func_body = untyped::Expr::new(untyped::ExprKind::Block(Block::new(*func_body)));
        let return_value = self.visit_block(&func_body);
        return_value
    }

    fn visit_identifier(&mut self, identifier: &untyped::Expr) -> Option<Value> {
        let identifier = if let untyped::ExprKind::Identifier(ident) = &identifier.kind {
            ident
        } else {
            panic!("Expected Identifier, found something else");
        };
        let ident = identifier.name.clone();
        let val = self.env.get(&ident);
        match val {
            Some(v) => Some(v.clone()),
            None => panic!("Undefined variable"),
        }
    }

    fn visit_return(&mut self, return_: &untyped::Expr) -> Option<Value> {
        let return_ = if let untyped::ExprKind::Return(return_) = &return_.kind {
            return_
        } else {
            panic!("Expected Return, found something else");
        };
        let val = self.visit_expr(&return_.value);
        val
    }

    fn visit_assignment(&mut self, assignment: &untyped::Expr) -> Option<Value> {
        let assignment = if let untyped::ExprKind::Assignment(assignment) = &assignment.kind {
            assignment
        } else {
            panic!("Expected Assignment, found something else");
        };
        let val = self.visit_expr(&assignment.value);
        self.env.insert(assignment.ident.name.clone(), val.unwrap());
        None
    }

    fn visit_loop(&mut self, loop_: &untyped::Expr) -> Option<Value> {
        let loop_ = if let untyped::ExprKind::Loop(loop_) = &loop_.kind {
            loop_
        } else {
            panic!("Expected Loop, found something else");
        };
        let mut cond = self.visit_expr(&loop_.cond);
        while let Some(Value::Bool(b)) = cond {
            if b {
                let loop_body =
                    untyped::Expr::new(untyped::ExprKind::Block(Block::new(*loop_.body.clone())));
                self.visit_block(&loop_body);
                cond = self.visit_expr(&loop_.cond);
            } else {
                break;
            }
        }
        None
    }

    fn visit_parameter(&mut self, parameter: &untyped::Expr) -> Option<Value> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::untyped::*;
    use crate::syntax::untyped_parser::parse;

    #[test]
    fn test_interpret_literal_int() {
        let mut interpreter = Interpreter::new();
        let source = "42";
        let ast = parse(source).unwrap();
        let result = interpreter.visit_expr(&ast[0]);
        assert_eq!(result, Some(Value::Int(42)));
    }

    #[test]
    fn test_interpret_literal_bool() {
        let mut interpreter = Interpreter::new();
        let source = "true";
        let ast = parse(source).unwrap();
        let result = interpreter.visit_expr(&ast[0]);
        assert_eq!(result, Some(Value::Bool(true)));
    }

    #[test]
    fn test_interpret_unary_expr() {
        let mut interpreter = Interpreter::new();
        let source = "-5";
        let ast = parse(source).unwrap();
        let result = interpreter.visit_expr(&ast[0]);
        assert_eq!(result, Some(Value::Int(-5)));
    }

    #[test]
    fn test_interpret_binary_expr() {
        let mut interpreter = Interpreter::new();
        let source = "3 + 5";
        let ast = parse(source).unwrap();
        let result = interpreter.visit_expr(&ast[0]);
        assert_eq!(result, Some(Value::Int(8)));
    }

    #[test]
    fn test_interpret_function_call() {
        let mut interpreter = Interpreter::new();
        let source = "
        def add(x, y) { return x + y }
        add(3, 4)
        ";
        let ast = parse(source).unwrap();
        let _ = interpreter.visit_expr(&ast[0]); // Define the function
        let result = interpreter.visit_expr(&ast[1]); // Call the function
        assert_eq!(result, Some(Value::Int(7)));
    }

    #[test]
    fn test_interpret_conditional() {
        let mut interpreter = Interpreter::new();
        let source = "
        if (true) {
            42
        } else {
            43
        }
        ";
        let ast = parse(source).unwrap();
        let result = interpreter.visit_expr(&ast[0]);
        assert_eq!(result, Some(Value::Int(42)));
    }

    #[test]
    fn test_interpret_assignment() {
        let mut interpreter = Interpreter::new();
        let source = "
        x = 42
        x
        ";
        let ast = parse(source).unwrap();
        let _ = interpreter.visit_expr(&ast[0]);
        let result = interpreter.visit_expr(&ast[1]);
        assert_eq!(result, Some(Value::Int(42)));
        let source = "
        n = 1
        n = n + 1
        n
        ";
        let ast = parse(source).unwrap();
        let _ = interpreter.visit_expr(&ast[0]);
        let _ = interpreter.visit_expr(&ast[1]);
        let result = interpreter.visit_expr(&ast[2]);
        assert_eq!(result, Some(Value::Int(2)));
    }

    #[test]
    fn test_interpret_loop() {
        let mut interpreter = Interpreter::new();
        let source = "
        x = 0
        while (x < 10) {
            x = x + 1
        }
        x
        ";
        let ast = parse(source).unwrap();
        let _ = interpreter.visit_expr(&ast[0]);
        let _ = interpreter.visit_expr(&ast[1]);
        let result = interpreter.visit_expr(&ast[2]);
        assert_eq!(result, Some(Value::Int(10)));
    }

    #[test]
    fn test_fib() {
        let mut interpreter = Interpreter::new();
        let source = "
    def factorial(n) {
        result = 1
        while (n > 1) {
            result = result * n
            n = n - 1
        }
        return result
    }
    factorial(5)
    ";
        let ast = parse(source).unwrap();
        dbg!("{:#?}", ast.clone());
        let _ = interpreter.visit_expr(&ast[0]);
        let result = interpreter.visit_expr(&ast[1]);
        assert_eq!(result, Some(Value::Int(120)));
    }
}
