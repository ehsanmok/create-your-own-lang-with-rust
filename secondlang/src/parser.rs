//! Parser for Secondlang
//!
//! Converts source code into a typed AST using the pest parser generator.

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{BinaryOp, Expr, Program, Stmt, TypedExpr, UnaryOp};
use crate::types::Type;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SecondlangParser;

/// Parse source code into a program (list of statements)
pub fn parse(source: &str) -> Result<Program, String> {
    let pairs = SecondlangParser::parse(Rule::Program, source)
        .map_err(|e| format!("Parse error: {}", e))?;

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
        Rule::Conditional | Rule::WhileLoop | Rule::Comparison => {
            Ok(Stmt::Expr(parse_expr(inner)?))
        }
        r => Err(format!("Unexpected statement rule: {:?}", r)),
    }
}

fn parse_function(pair: Pair<Rule>) -> Result<Stmt, String> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut params: Vec<(String, Type)> = Vec::new();
    let mut return_type = Type::Unknown;
    let mut body = Vec::new();

    for item in inner {
        match item.as_rule() {
            Rule::TypedParam => {
                let mut param_inner = item.into_inner();
                let param_name = param_inner.next().unwrap().as_str().to_string();
                let param_type = parse_type(param_inner.next().unwrap())?;
                params.push((param_name, param_type));
            }
            Rule::ReturnType => {
                let type_pair = item.into_inner().next().unwrap();
                return_type = parse_type(type_pair)?;
            }
            Rule::Block => {
                body = parse_block(item)?;
            }
            _ => {}
        }
    }

    Ok(Stmt::Function {
        name,
        params,
        return_type,
        body,
    })
}

fn parse_type(pair: Pair<Rule>) -> Result<Type, String> {
    match pair.as_rule() {
        Rule::Type => parse_type(pair.into_inner().next().unwrap()),
        Rule::IntType => Ok(Type::Int),
        Rule::BoolType => Ok(Type::Bool),
        r => Err(format!("Unexpected type rule: {:?}", r)),
    }
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

    let mut type_ann = None;
    let mut value_pair = None;

    for item in inner {
        match item.as_rule() {
            Rule::Type => {
                type_ann = Some(parse_type(item)?);
            }
            _ => {
                value_pair = Some(item);
            }
        }
    }

    let value = parse_expr(value_pair.unwrap())?;
    Ok(Stmt::Assignment {
        name,
        type_ann,
        value,
    })
}

fn parse_expr(pair: Pair<Rule>) -> Result<TypedExpr, String> {
    let expr = match pair.as_rule() {
        Rule::Expr => {
            let inner = pair.into_inner().next().unwrap();
            return parse_expr(inner);
        }
        Rule::Conditional => parse_conditional(pair)?,
        Rule::WhileLoop => parse_while(pair)?,
        Rule::Comparison => parse_binary(pair)?,
        Rule::Additive => parse_binary(pair)?,
        Rule::Multiplicative => parse_binary(pair)?,
        Rule::Unary => return parse_unary(pair),
        Rule::Call => return parse_call(pair),
        Rule::Literal => parse_literal(pair)?,
        Rule::Int => Expr::Int(pair.as_str().parse().unwrap()),
        Rule::Bool => Expr::Bool(pair.as_str() == "true"),
        Rule::Identifier => Expr::Var(pair.as_str().to_string()),
        Rule::Block => {
            let stmts = parse_block(pair)?;
            Expr::Block(stmts)
        }
        r => return Err(format!("Unexpected expression rule: {:?}", r)),
    };
    Ok(TypedExpr::unknown(expr))
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
        left = TypedExpr::unknown(Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        });
    }

    Ok(left.expr)
}

fn parse_unary(pair: Pair<Rule>) -> Result<TypedExpr, String> {
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
            Ok(TypedExpr::unknown(Expr::Unary {
                op,
                expr: Box::new(expr),
            }))
        }
        _ => parse_expr(first),
    }
}

fn parse_call(pair: Pair<Rule>) -> Result<TypedExpr, String> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    let mut expr = parse_expr(first)?;

    // Check for CallArgs (function call)
    for call_args in inner {
        if call_args.as_rule() == Rule::CallArgs {
            let args: Vec<TypedExpr> = call_args
                .into_inner()
                .map(|p| parse_expr(p))
                .collect::<Result<_, _>>()?;

            if let Expr::Var(name) = expr.expr {
                expr = TypedExpr::unknown(Expr::Call { name, args });
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
    fn test_parse_typed_function() {
        let source = "def add(a: int, b: int) -> int { return a + b }";
        let program = parse(source).unwrap();
        if let Stmt::Function {
            name,
            params,
            return_type,
            ..
        } = &program[0]
        {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], ("a".to_string(), Type::Int));
            assert_eq!(params[1], ("b".to_string(), Type::Int));
            assert_eq!(*return_type, Type::Int);
        } else {
            panic!("Expected Function");
        }
    }

    #[test]
    fn test_parse_typed_assignment() {
        let source = "x: int = 42";
        let program = parse(source).unwrap();
        if let Stmt::Assignment {
            name,
            type_ann,
            value: _,
        } = &program[0]
        {
            assert_eq!(name, "x");
            assert_eq!(*type_ann, Some(Type::Int));
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_fibonacci() {
        let source = r#"
            def fib(n: int) -> int {
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
