use crate::ast::untyped::*;
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

impl Visitor<Option<Value>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Option<Value> {
        match &expr.kind {
            ExprKind::Literal(lk) => self.visit_literal(lk),
            ExprKind::UnaryExpr(ue) => self.visit_unary_expr(ue),
            ExprKind::BinaryExpr(be) => self.visit_binary_expr(be),
            ExprKind::Conditional(cond) => self.visit_conditional(cond),
            ExprKind::Function(func) => self.visit_function(func),
            ExprKind::Call(call) => self.visit_call(call),
            ExprKind::Identifier(ident) => self.visit_identifier(ident),
            ExprKind::Return(return_) => self.visit_return(return_),
            ExprKind::Assignment(assignment) => self.visit_assignment(assignment),
            ExprKind::Loop(loop_) => self.visit_loop(loop_),
        }
    }

    fn visit_literal(&mut self, literal: &LiteralKind) -> Option<Value> {
        match literal {
            LiteralKind::Int(i) => Some(Value::Int(*i)),
            LiteralKind::Bool(b) => Some(Value::Bool(*b)),
        }
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Option<Value> {
        let operand = self.visit_expr(&unary_expr.child);
        match unary_expr.op {
            UnaryOp::Plus => operand,
            UnaryOp::Minus => match operand {
                Some(Value::Int(i)) => Some(Value::Int(-i)),
                _ => panic!("Unary minus applied to non-integer value"),
            },
        }
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr) -> Option<Value> {
        let left = self.visit_expr(&binary_expr.lhs);
        let right = self.visit_expr(&binary_expr.rhs);
        match binary_expr.op {
            BinaryOp::Add => match (left, right) {
                (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Int(l + r)),
                _ => panic!("Addition applied to non-integer values"),
            },
            BinaryOp::Sub => match (left, right) {
                (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Int(l - r)),
                _ => panic!("Subtraction applied to non-integer values"),
            },
            BinaryOp::LessThan => match (left, right) {
                (Some(Value::Int(l)), Some(Value::Int(r))) => Some(Value::Bool(l < r)),
                _ => panic!("Less-than applied to non-integer values"),
            },
        }
    }

    fn visit_conditional(&mut self, conditional: &Conditional) -> Option<Value> {
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
    }

    fn visit_function(&mut self, function: &Function) -> Option<Value> {
        let fname = function.ident.name.clone();
        self.env.insert(fname, Value::Function(function.clone()));
        None
    }

    fn visit_call(&mut self, call: &Call) -> Option<Value> {
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
        let return_value = self.visit_expr(&func_body);
        return_value
    }

    fn visit_identifier(&mut self, identifier: &Identifier) -> Option<Value> {
        let ident = identifier.name.clone();
        let val = self.env.get(&ident);
        match val {
            Some(v) => Some(v.clone()),
            None => panic!("Undefined variable"),
        }
    }

    fn visit_return(&mut self, return_: &Return) -> Option<Value> {
        let val = self.visit_expr(&return_.value);
        val
    }

    fn visit_assignment(&mut self, assignment: &Assignment) -> Option<Value> {
        let val = self.visit_expr(&assignment.value);
        self.env.insert(assignment.ident.name.clone(), val.unwrap());
        None
    }

    fn visit_loop(&mut self, loop_: &Loop) -> Option<Value> {
        let mut cond = self.visit_expr(&loop_.cond);
        while let Some(Value::Bool(b)) = cond {
            if b {
                self.visit_expr(&loop_.body);
                cond = self.visit_expr(&loop_.cond);
            } else {
                break;
            }
        }
        None
    }

    fn visit_parameter(&mut self, parameter: &Parameter) -> Option<Value> {
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
}
