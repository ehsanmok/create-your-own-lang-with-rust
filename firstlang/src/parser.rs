//! Parser for Firstlang
//!
//! Converts source code into an AST using the pest parser generator.

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FirstlangParser;

/// Parse source code into a program (list of statements)
pub fn parse(source: &str) -> Result<Program, String> {
    let pairs =
        FirstlangParser::parse(Rule::Program, source).map_err(|e| format!("Parse error: {}", e))?;

    let mut program = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::Stmt => {
                program.push(parse_stmt(pair)?);
            }
            Rule::EOI => {}
            _ => {}
        }
    }
    Ok(program)
}

fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt, String> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::Function => parse_function(inner),
        Rule::Return => parse_return(inner),
        Rule::Assignment => parse_assignment(inner),
        Rule::Expr => Ok(Stmt::Expr(parse_expr(inner)?)),
        // Handle direct expression rules that might appear
        Rule::Conditional | Rule::WhileLoop | Rule::Comparison => {
            Ok(Stmt::Expr(parse_expr(inner)?))
        }
        r => Err(format!("Unexpected statement rule: {:?}", r)),
    }
}

fn parse_function(pair: Pair<Rule>) -> Result<Stmt, String> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut params = Vec::new();
    let mut body = Vec::new();

    for item in inner {
        match item.as_rule() {
            Rule::Identifier => {
                params.push(item.as_str().to_string());
            }
            Rule::Block => {
                body = parse_block(item)?;
            }
            _ => {}
        }
    }

    Ok(Stmt::Function { name, params, body })
}

fn parse_block(pair: Pair<Rule>) -> Result<Vec<Stmt>, String> {
    let mut stmts = Vec::new();
    for item in pair.into_inner() {
        if item.as_rule() == Rule::Stmt {
            stmts.push(parse_stmt(item)?);
        }
    }
    Ok(stmts)
}

fn parse_return(pair: Pair<Rule>) -> Result<Stmt, String> {
    let expr = pair.into_inner().next().unwrap();
    Ok(Stmt::Return(parse_expr(expr)?))
}

fn parse_assignment(pair: Pair<Rule>) -> Result<Stmt, String> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let value = parse_expr(inner.next().unwrap())?;
    Ok(Stmt::Assignment { name, value })
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr, String> {
    match pair.as_rule() {
        Rule::Expr => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        Rule::Conditional => parse_conditional(pair),
        Rule::WhileLoop => parse_while(pair),
        Rule::Comparison => parse_binary(pair),
        Rule::Additive => parse_binary(pair),
        Rule::Multiplicative => parse_binary(pair),
        Rule::Unary => parse_unary(pair),
        Rule::Call => parse_call(pair),
        Rule::Literal => parse_literal(pair),
        Rule::Int => Ok(Expr::Int(pair.as_str().parse().unwrap())),
        Rule::Bool => Ok(Expr::Bool(pair.as_str() == "true")),
        Rule::Identifier => Ok(Expr::Var(pair.as_str().to_string())),
        Rule::Block => {
            let stmts = parse_block(pair)?;
            Ok(Expr::Block(stmts))
        }
        r => Err(format!("Unexpected expression rule: {:?}", r)),
    }
}

fn parse_conditional(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut inner = pair.into_inner();
    let cond = Box::new(parse_expr(inner.next().unwrap())?);
    let then_branch = parse_block(inner.next().unwrap())?;
    let else_branch = parse_block(inner.next().unwrap())?;
    Ok(Expr::If {
        cond,
        then_branch,
        else_branch,
    })
}

fn parse_while(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut inner = pair.into_inner();
    let cond = Box::new(parse_expr(inner.next().unwrap())?);
    let body = parse_block(inner.next().unwrap())?;
    Ok(Expr::While { cond, body })
}

fn parse_binary(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut inner = pair.into_inner();
    let mut left = parse_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "+" => BinaryOp::Add,
            "-" => BinaryOp::Sub,
            "*" => BinaryOp::Mul,
            "/" => BinaryOp::Div,
            "%" => BinaryOp::Mod,
            "<" => BinaryOp::Lt,
            ">" => BinaryOp::Gt,
            "<=" => BinaryOp::Le,
            ">=" => BinaryOp::Ge,
            "==" => BinaryOp::Eq,
            "!=" => BinaryOp::Ne,
            s => return Err(format!("Unknown operator: {}", s)),
        };
        let right = parse_expr(inner.next().unwrap())?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_unary(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    match first.as_rule() {
        Rule::UnaryOp => {
            let op = match first.as_str() {
                "-" => UnaryOp::Neg,
                "!" => UnaryOp::Not,
                s => return Err(format!("Unknown unary operator: {}", s)),
            };
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
            })
        }
        // No unary operator, just parse the inner expression
        _ => parse_expr(first),
    }
}

fn parse_call(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    // Parse the primary expression (function name or parenthesized expr)
    let mut expr = parse_expr(first)?;

    // Check for CallArgs (function call)
    for call_args in inner {
        if call_args.as_rule() == Rule::CallArgs {
            // Parse arguments inside the CallArgs
            let args: Vec<Expr> = call_args
                .into_inner()
                .map(|p| parse_expr(p))
                .collect::<Result<_, _>>()?;

            // This is a function call
            if let Expr::Var(name) = expr {
                expr = Expr::Call { name, args };
            } else {
                return Err("Can only call named functions".to_string());
            }
        }
    }

    Ok(expr)
}

fn parse_literal(pair: Pair<Rule>) -> Result<Expr, String> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::Int => Ok(Expr::Int(inner.as_str().parse().unwrap())),
        Rule::Bool => Ok(Expr::Bool(inner.as_str() == "true")),
        r => Err(format!("Unexpected literal rule: {:?}", r)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let program = parse("42").unwrap();
        assert_eq!(program.len(), 1);
        if let Stmt::Expr(Expr::Int(n)) = &program[0] {
            assert_eq!(*n, 42);
        } else {
            panic!("Expected Int literal");
        }
    }

    #[test]
    fn test_parse_bool() {
        let program = parse("true").unwrap();
        if let Stmt::Expr(Expr::Bool(b)) = &program[0] {
            assert!(*b);
        } else {
            panic!("Expected Bool literal");
        }
    }

    #[test]
    fn test_parse_binary() {
        let program = parse("1 + 2").unwrap();
        if let Stmt::Expr(Expr::Binary { op, .. }) = &program[0] {
            assert_eq!(*op, BinaryOp::Add);
        } else {
            panic!("Expected Binary expression");
        }
    }

    #[test]
    fn test_parse_assignment() {
        let program = parse("x = 42").unwrap();
        if let Stmt::Assignment { name, value } = &program[0] {
            assert_eq!(name, "x");
            assert_eq!(*value, Expr::Int(42));
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_function() {
        let program = parse("def add(a, b) { return a + b }").unwrap();
        if let Stmt::Function { name, params, .. } = &program[0] {
            assert_eq!(name, "add");
            assert_eq!(params, &["a", "b"]);
        } else {
            panic!("Expected Function");
        }
    }

    #[test]
    fn test_parse_call() {
        let program = parse("add(1, 2)").unwrap();
        if let Stmt::Expr(Expr::Call { name, args }) = &program[0] {
            assert_eq!(name, "add");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_conditional() {
        let program = parse("if (x < 10) { 1 } else { 2 }").unwrap();
        if let Stmt::Expr(Expr::If { .. }) = &program[0] {
            // Successfully parsed
        } else {
            panic!("Expected If expression");
        }
    }

    #[test]
    fn test_parse_while() {
        let program = parse("while (x < 10) { x = x + 1 }").unwrap();
        if let Stmt::Expr(Expr::While { body, .. }) = &program[0] {
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected While expression");
        }
    }

    #[test]
    fn test_parse_fibonacci() {
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
        let program = parse(source).unwrap();
        assert_eq!(program.len(), 2);
    }
}
