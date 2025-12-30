//! Tree-walking Interpreter for Firstlang
//!
//! This interpreter executes the AST directly, supporting:
//! - Variables and assignment
//! - Functions with parameters
//! - Recursion (via proper call stack)
//! - Control flow (if/else, while)

use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};

/// Runtime values in our language
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    /// A function value stores its parameter names and body
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    /// Unit value (returned from statements with no value)
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Function { params, .. } => write!(f, "<function({})>", params.join(", ")),
            Value::Unit => write!(f, "()"),
        }
    }
}

/// An environment frame (for local variables in a function call)
#[derive(Debug, Clone)]
struct Frame {
    locals: HashMap<String, Value>,
}

impl Frame {
    fn new() -> Self {
        Frame {
            locals: HashMap::new(),
        }
    }
}

/// The interpreter state
pub struct Interpreter {
    /// Global environment (for functions and global variables)
    globals: HashMap<String, Value>,
    /// Call stack of local environments (for recursion support)
    call_stack: Vec<Frame>,
}

/// Control flow signals for the interpreter
enum ControlFlow {
    /// Normal execution continues
    Continue(Value),
    /// Return statement encountered
    Return(Value),
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            call_stack: vec![Frame::new()], // Start with one global frame
        }
    }

    /// Run a complete program
    pub fn run(&mut self, program: &Program) -> Result<Value, String> {
        let mut result = Value::Unit;
        for stmt in program {
            match self.exec_stmt(stmt)? {
                ControlFlow::Continue(v) => result = v,
                ControlFlow::Return(v) => return Ok(v),
            }
        }
        Ok(result)
    }

    /// Execute a single statement
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<ControlFlow, String> {
        match stmt {
            Stmt::Function { name, params, body } => {
                // Store the function in globals
                self.globals.insert(
                    name.clone(),
                    Value::Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(ControlFlow::Continue(Value::Unit))
            }

            Stmt::Return(expr) => {
                let value = self.eval_expr(expr)?;
                Ok(ControlFlow::Return(value))
            }

            Stmt::Assignment { name, value } => {
                let val = self.eval_expr(value)?;
                // Assign to the current frame (local scope)
                self.current_frame_mut().locals.insert(name.clone(), val);
                Ok(ControlFlow::Continue(Value::Unit))
            }

            Stmt::Expr(expr) => {
                let value = self.eval_expr(expr)?;
                Ok(ControlFlow::Continue(value))
            }
        }
    }

    /// Evaluate an expression
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Int(n) => Ok(Value::Int(*n)),

            Expr::Bool(b) => Ok(Value::Bool(*b)),

            Expr::Var(name) => self.lookup_var(name),

            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match (op, val) {
                    (UnaryOp::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
                    (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    (op, val) => Err(format!("Cannot apply {:?} to {:?}", op, val)),
                }
            }

            Expr::Binary { op, left, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary_op(*op, l, r)
            }

            Expr::Call { name, args } => {
                // Look up the function
                let func = self.lookup_var(name)?;

                if let Value::Function { params, body } = func {
                    // Evaluate arguments
                    let arg_values: Vec<Value> = args
                        .iter()
                        .map(|a| self.eval_expr(a))
                        .collect::<Result<_, _>>()?;

                    // Check arity
                    if params.len() != arg_values.len() {
                        return Err(format!(
                            "Function {} expects {} arguments, got {}",
                            name,
                            params.len(),
                            arg_values.len()
                        ));
                    }

                    // Create new frame for this call
                    let mut frame = Frame::new();
                    for (param, arg) in params.iter().zip(arg_values) {
                        frame.locals.insert(param.clone(), arg);
                    }

                    // Push the new frame onto the call stack
                    self.call_stack.push(frame);

                    // Execute the function body
                    let mut result = Value::Unit;
                    for stmt in &body {
                        match self.exec_stmt(stmt)? {
                            ControlFlow::Continue(v) => result = v,
                            ControlFlow::Return(v) => {
                                // Pop the frame before returning
                                self.call_stack.pop();
                                return Ok(v);
                            }
                        }
                    }

                    // Pop the frame after normal completion
                    self.call_stack.pop();
                    Ok(result)
                } else {
                    Err(format!("{} is not a function", name))
                }
            }

            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(cond)?;
                if let Value::Bool(b) = cond_val {
                    let branch = if b { then_branch } else { else_branch };
                    let mut result = Value::Unit;
                    for stmt in branch {
                        match self.exec_stmt(stmt)? {
                            ControlFlow::Continue(v) => result = v,
                            ControlFlow::Return(v) => return Ok(v),
                        }
                    }
                    Ok(result)
                } else {
                    Err(format!("Condition must be boolean, got {:?}", cond_val))
                }
            }

            Expr::While { cond, body } => {
                loop {
                    let cond_val = self.eval_expr(cond)?;
                    if let Value::Bool(b) = cond_val {
                        if !b {
                            break;
                        }
                        for stmt in body {
                            match self.exec_stmt(stmt)? {
                                ControlFlow::Continue(_) => {}
                                ControlFlow::Return(v) => return Ok(v),
                            }
                        }
                    } else {
                        return Err(format!(
                            "While condition must be boolean, got {:?}",
                            cond_val
                        ));
                    }
                }
                Ok(Value::Unit)
            }

            Expr::Block(stmts) => {
                let mut result = Value::Unit;
                for stmt in stmts {
                    match self.exec_stmt(stmt)? {
                        ControlFlow::Continue(v) => result = v,
                        ControlFlow::Return(v) => return Ok(v),
                    }
                }
                Ok(result)
            }
        }
    }

    /// Look up a variable (check local frames first, then globals)
    fn lookup_var(&self, name: &str) -> Result<Value, String> {
        // Check the current frame first (local variables)
        if let Some(val) = self.current_frame().locals.get(name) {
            return Ok(val.clone());
        }

        // Check globals (functions and global variables)
        if let Some(val) = self.globals.get(name) {
            return Ok(val.clone());
        }

        Err(format!("Undefined variable: {}", name))
    }

    /// Get a reference to the current (top) frame
    fn current_frame(&self) -> &Frame {
        self.call_stack
            .last()
            .expect("Call stack should never be empty")
    }

    /// Get a mutable reference to the current (top) frame
    fn current_frame_mut(&mut self) -> &mut Frame {
        self.call_stack
            .last_mut()
            .expect("Call stack should never be empty")
    }

    /// Evaluate a binary operation
    fn eval_binary_op(&self, op: BinaryOp, left: Value, right: Value) -> Result<Value, String> {
        match (op, &left, &right) {
            // Arithmetic operations (integers only)
            (BinaryOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (BinaryOp::Sub, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (BinaryOp::Mul, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (BinaryOp::Div, Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (BinaryOp::Mod, Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            }

            // Comparison operations (integers)
            (BinaryOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (BinaryOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (BinaryOp::Le, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (BinaryOp::Ge, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (BinaryOp::Eq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (BinaryOp::Ne, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),

            // Boolean equality
            (BinaryOp::Eq, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            (BinaryOp::Ne, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),

            // Type mismatch
            _ => Err(format!(
                "Cannot apply {:?} to {:?} and {:?}",
                op, left, right
            )),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn run(source: &str) -> Result<Value, String> {
        let program = parse(source)?;
        let mut interpreter = Interpreter::new();
        interpreter.run(&program)
    }

    #[test]
    fn test_literal() {
        assert_eq!(run("42").unwrap(), Value::Int(42));
        assert_eq!(run("true").unwrap(), Value::Bool(true));
        assert_eq!(run("false").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(run("1 + 2").unwrap(), Value::Int(3));
        assert_eq!(run("10 - 3").unwrap(), Value::Int(7));
        assert_eq!(run("4 * 5").unwrap(), Value::Int(20));
        assert_eq!(run("15 / 3").unwrap(), Value::Int(5));
        assert_eq!(run("17 % 5").unwrap(), Value::Int(2));
    }

    #[test]
    fn test_comparison() {
        assert_eq!(run("1 < 2").unwrap(), Value::Bool(true));
        assert_eq!(run("2 > 1").unwrap(), Value::Bool(true));
        assert_eq!(run("1 == 1").unwrap(), Value::Bool(true));
        assert_eq!(run("1 != 2").unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_variables() {
        assert_eq!(run("x = 42\nx").unwrap(), Value::Int(42));
    }

    #[test]
    fn test_function() {
        let source = r#"
            def add(a, b) {
                return a + b
            }
            add(3, 4)
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(7));
    }

    #[test]
    fn test_conditional() {
        let source = r#"
            if (true) {
                42
            } else {
                0
            }
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(42));

        let source = r#"
            if (false) {
                42
            } else {
                0
            }
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(0));
    }

    #[test]
    fn test_while_loop() {
        let source = r#"
            x = 0
            while (x < 5) {
                x = x + 1
            }
            x
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(5));
    }

    #[test]
    fn test_factorial_iterative() {
        let source = r#"
            def factorial(n) {
                result = 1
                while (n > 1) {
                    result = result * n
                    n = n - 1
                }
                return result
            }
            factorial(5)
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(120));
    }

    #[test]
    fn test_factorial_recursive() {
        let source = r#"
            def factorial(n) {
                if (n <= 1) {
                    return 1
                } else {
                    return n * factorial(n - 1)
                }
            }
            factorial(5)
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(120));
    }

    #[test]
    fn test_fibonacci_recursive() {
        let source = r#"
            def fib(n) {
                if (n < 2) {
                    return n
                } else {
                    return fib(n - 1) + fib(n - 2)
                }
            }
            fib(10)
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(55));
    }

    #[test]
    fn test_fibonacci_iterative() {
        let source = r#"
            def fib(n) {
                if (n < 2) {
                    return n
                } else {
                    a = 0
                    b = 1
                    i = 2
                    while (i <= n) {
                        temp = a + b
                        a = b
                        b = temp
                        i = i + 1
                    }
                    return b
                }
            }
            fib(10)
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(55));
    }

    #[test]
    fn test_nested_calls() {
        let source = r#"
            def double(x) {
                return x * 2
            }
            def quadruple(x) {
                return double(double(x))
            }
            quadruple(5)
        "#;
        assert_eq!(run(source).unwrap(), Value::Int(20));
    }
}
