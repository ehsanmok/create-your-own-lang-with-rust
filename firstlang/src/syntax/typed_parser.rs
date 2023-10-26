#![allow(clippy::upper_case_acronyms)]

use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};

use std::result::Result as StdResult;

use crate::ast::typed as ast;

#[derive(Parser)]
#[grammar = "syntax/grammar_typed.pest"]
struct TypedParser;

pub fn parse(source: &str) -> StdResult<Vec<ast::Expr>, Error<Rule>> {
    let pairs = TypedParser::parse(Rule::File, source)?;
    pairs
        .into_iter()
        .filter_map(|pair| {
            if let Rule::Expr = pair.as_rule() {
                Some(build_ast_from_expr(pair))
            } else {
                None
            }
        })
        .collect()
}

fn build_ast_from_expr(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().expect("Expected inner Rule")),
        Rule::Function => parse_function(pair),
        Rule::Call => parse_call(pair),
        Rule::Loop => parse_loop(pair),
        Rule::Conditional => parse_conditional(pair),
        Rule::Assignment => parse_assignment(pair),
        Rule::Return => parse_return(pair),
        Rule::BinaryExpr => parse_binary_expr(pair),
        Rule::UnaryExpr => parse_unary_expr(pair),
        Rule::Literal => parse_literal(pair),
        Rule::Identifier => parse_identifier(pair),
        Rule::Parameter => parse_parameter(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn parse_type(pair: Pair<Rule>) -> StdResult<ast::Type, Error<Rule>> {
    match pair.as_rule() {
        Rule::BuiltinTypes => parse_builtin_types(pair),
        Rule::ReturnType => parse_return_type(pair),
        unknown => panic!("Unknown type: {:?}", unknown),
    }
}

fn parse_builtin_types(pair: Pair<Rule>) -> StdResult<ast::Type, Error<Rule>> {
    let inner = pair.into_inner().next().expect("Expected inner rule");
    match inner.as_rule() {
        Rule::IntType => Ok(ast::Type::Scalar(ast::ScalarKind::Int)),
        Rule::BoolType => Ok(ast::Type::Scalar(ast::ScalarKind::Bool)),
        unknown => panic!("Unknown builtin type: {:?}", unknown),
    }
}

fn parse_return_type(pair: Pair<Rule>) -> StdResult<ast::Type, Error<Rule>> {
    let inner = pair
        .into_inner()
        .next()
        .expect("Expected inner rule for ReturnType");
    parse_builtin_types(inner)
}

fn parse_function_type(pair: Pair<Rule>) -> StdResult<ast::Type, Error<Rule>> {
    let mut pairs = pair.into_inner();
    let params_pair = pairs.next().expect("Expected parameters pair");
    let params: Vec<ast::Type> = params_pair
        .into_inner()
        .map(|param| parse_type(param).expect("Expected type for parameter"))
        .collect();

    let return_type = pairs
        .next()
        .map(|return_pair| parse_type(return_pair).expect("Expected return type"))
        .unwrap_or(ast::Type::Unknown); // Handle case where return type is omitted

    Ok(ast::Type::Function(params, Box::new(return_type)))
}

fn parse_function(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let fn_ident = build_ast_from_expr(pair.next().unwrap())?;
    let mut params = vec![];
    let mut body_exprs = vec![];
    let mut out_type = ast::Type::Unknown;

    loop {
        let param_pair = pair.next().unwrap();
        if let Rule::Parameter = param_pair.as_rule() {
            let mut inner = param_pair.into_inner();
            let param_ident = build_ast_from_expr(inner.next().unwrap())?;
            let param_type = parse_type(inner.next().unwrap())?;

            params.push(ast::Parameter {
                ident: param_ident.to_identifier().unwrap(),
                ty: param_type,
            });
        } else if let Rule::ReturnType = param_pair.as_rule() {
            out_type = parse_type(param_pair)?;
        } else {
            // If we encounter something that's not a parameter or a return type, we assume the rest is the function body
            body_exprs.push(build_ast_from_expr(param_pair)?);
            break;
        }
    }

    // include ty signature
    let ty_sig = ast::Type::Function(
        params.iter().map(|p| p.clone().ty).collect(),
        Box::new(out_type),
    );
    // Extract the body expressions
    for expr_pair in pair {
        body_exprs.push(build_ast_from_expr(expr_pair)?);
    }

    Ok(ast::Expr::new(
        ast::ExprKind::Function(ast::Function::new(
            fn_ident.to_identifier().unwrap(),
            params,
            Box::new(body_exprs),
        )),
        ty_sig,
    ))
}

fn parse_call(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut inner_pairs = pair.into_inner();
    let fn_ident = build_ast_from_expr(inner_pairs.next().unwrap())?;
    let mut args = vec![];

    for arg_pair in inner_pairs {
        if let Rule::Expr = arg_pair.as_rule() {
            args.push(build_ast_from_expr(arg_pair)?);
        }
    }

    Ok(ast::Expr::new(
        ast::ExprKind::Call(ast::Call::new(fn_ident.to_identifier().unwrap(), args)),
        ast::Type::Unknown,
    ))
}

fn parse_loop(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let cond = build_ast_from_expr(pair.next().unwrap())?;
    let mut body = Vec::new();
    for expr_pair in pair {
        body.push(build_ast_from_expr(expr_pair)?);
    }
    Ok(ast::Expr::new(
        ast::ExprKind::Loop(ast::Loop::new(Box::new(cond), Box::new(body))),
        ast::Type::Unit,
    ))
}

fn parse_conditional(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let cond = build_ast_from_expr(pair.next().unwrap())?;
    let on_true = build_ast_from_expr(pair.next().unwrap())?;
    let on_false = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(
        ast::ExprKind::Conditional(ast::Conditional::new(
            Box::new(cond),
            Box::new(on_true),
            Box::new(on_false),
        )),
        ast::Type::Unknown,
    ))
}

fn parse_assignment(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let ident = build_ast_from_expr(pair.next().unwrap())?;
    let value = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(
        ast::ExprKind::Assignment(ast::Assignment::new(
            ident.to_identifier().unwrap(),
            Box::new(value),
        )),
        ast::Type::Unknown,
    ))
}

fn parse_return(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let value = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(
        ast::ExprKind::Return(ast::Return::new(Box::new(value))),
        ast::Type::Unknown,
    ))
}

fn parse_literal(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::Int => {
            let int = pair.as_str().trim().parse::<i64>().unwrap();
            Ok(ast::Expr::new(
                ast::ExprKind::Literal(ast::LiteralKind::Int(int)),
                ast::Type::Scalar(ast::ScalarKind::Int),
            ))
        }
        Rule::Bool => {
            let b = pair.as_str().trim().parse::<bool>().unwrap();
            Ok(ast::Expr::new(
                ast::ExprKind::Literal(ast::LiteralKind::Bool(b)),
                ast::Type::Scalar(ast::ScalarKind::Bool),
            ))
        }
        unknown => panic!("Unknown literal: {:?}", unknown),
    }
}

fn parse_identifier(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    Ok(ast::Expr::new(
        ast::ExprKind::Identifier(ast::Identifier::new(pair.as_str().to_string())),
        ast::Type::Unknown,
    ))
}

fn parse_unary_expr(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let pairs = pair.collect::<Vec<Pair<Rule>>>();
    // If only one pair is present, it's just the Term, not an actual unary operation.
    if pairs.len() == 1 {
        return build_ast_from_term(pairs[0].clone());
    } else {
        let op = Some(pairs[0].clone());
        let child = pairs[1].clone();
        let child = build_ast_from_term(child)?;
        return Ok(parse_unary_expr_inner(op, child));
    }
}

fn parse_unary_expr_inner(pair: Option<Pair<Rule>>, child: ast::Expr) -> ast::Expr {
    if let Some(op_pair) = pair {
        let op = match op_pair.as_str() {
            "+" => ast::UnaryOp::Plus,
            "-" => ast::UnaryOp::Minus,
            _ => unreachable!(),
        };
        return ast::Expr::new(
            ast::ExprKind::UnaryExpr(ast::UnaryExpr::new(op, Box::new(child))),
            ast::Type::Unknown,
        );
    }
    child
}

fn parse_binary_expr(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let lhspair = pair.next().unwrap();
    let lhs = build_ast_from_term(lhspair)?;
    let op = pair.next().unwrap();
    let rhspair = pair.next().unwrap();
    let rhs = build_ast_from_term(rhspair)?;
    Ok(parse_binary_expr_inner(op, lhs, rhs))
}

fn parse_binary_expr_inner(pair: Pair<Rule>, lhs: ast::Expr, rhs: ast::Expr) -> ast::Expr {
    let op = match pair.as_str() {
        "+" => ast::BinaryOp::Add,
        "-" => ast::BinaryOp::Sub,
        "*" => ast::BinaryOp::Mul,
        "<" => ast::BinaryOp::LessThan,
        ">" => ast::BinaryOp::GreaterThan,
        _ => unreachable!(),
    };
    ast::Expr::new(
        ast::ExprKind::BinaryExpr(ast::BinaryExpr::new(op, Box::new(lhs), Box::new(rhs))),
        ast::Type::Unknown,
    )
}

fn build_ast_from_term(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::Literal => parse_literal(pair.into_inner().next().unwrap()),
        Rule::Identifier => parse_identifier(pair),
        Rule::Call => parse_call(pair),
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_parameter(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let ident = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(
        ast::ExprKind::Identifier(ident.to_identifier().unwrap()),
        ast::Type::Unknown,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::typed::*;
    use crate::syntax::typed_parser::parse;

    #[test]
    fn test_parse_identifier() {
        let source = "identifier";
        let result = parse(source).unwrap();
        assert_eq!(
            result[0],
            Expr::new(
                ExprKind::Identifier(Identifier::new(String::from("identifier"))),
                ast::Type::Unknown
            )
        );
    }

    #[test]
    fn test_parse_literal_int() {
        let source = "42";
        let result = parse(source).unwrap();
        assert_eq!(
            result[0],
            Expr::new(
                ExprKind::Literal(LiteralKind::Int(42)),
                ast::Type::Scalar(ScalarKind::Int)
            )
        );
    }

    #[test]
    fn test_parse_literal_bool() {
        let source = "true";
        let result = parse(source).unwrap();
        assert_eq!(
            result[0],
            Expr::new(
                ExprKind::Literal(LiteralKind::Bool(true)),
                ast::Type::Scalar(ScalarKind::Bool)
            )
        );
    }

    #[test]
    fn test_parse_unary_expr() {
        let source = "-42";
        let result = parse(source).unwrap();
        assert_eq!(
            result[0],
            Expr::new(
                ExprKind::UnaryExpr(UnaryExpr::new(
                    UnaryOp::Minus,
                    Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(42)),
                        ast::Type::Scalar(ScalarKind::Int)
                    ))
                )),
                ast::Type::Unknown
            )
        );
    }

    #[test]
    fn test_parse_binary_expr() {
        let source = "3 + 5";
        let result = parse(source).unwrap();
        assert_eq!(
            result[0],
            Expr::new(
                ExprKind::BinaryExpr(BinaryExpr::new(
                    BinaryOp::Add,
                    Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(3)),
                        Type::Scalar(ScalarKind::Int)
                    )),
                    Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(5)),
                        Type::Scalar(ScalarKind::Int)
                    )),
                )),
                ast::Type::Unknown
            )
        );
        let source = "3 * 5";
        let result = parse(source).unwrap();
        assert_eq!(
            result[0],
            Expr::new(
                ExprKind::BinaryExpr(BinaryExpr::new(
                    BinaryOp::Mul,
                    Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(3)),
                        Type::Scalar(ScalarKind::Int)
                    )),
                    Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(5)),
                        Type::Scalar(ScalarKind::Int)
                    )),
                )),
                Type::Unknown
            )
        );
    }

    #[test]
    fn test_parse_function() {
        let source = "def myFunc(param1: int, param2: int) -> int { return 42 }";
        let result = parse(source).unwrap();
        let expected = Expr::new(
            ExprKind::Function(Function::new(
                Identifier::new(String::from("myFunc")),
                vec![
                    Parameter::new(
                        Identifier::new(String::from("param1")),
                        ast::Type::Scalar(ScalarKind::Int),
                    ),
                    Parameter::new(
                        Identifier::new(String::from("param2")),
                        ast::Type::Scalar(ScalarKind::Int),
                    ),
                ],
                Box::new(vec![Expr::new(
                    ExprKind::Return(Return::new(Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(42)),
                        Type::Scalar(ScalarKind::Int),
                    )))),
                    Type::Unknown,
                )]),
            )),
            Type::Function(
                vec![Type::Scalar(ScalarKind::Int), Type::Scalar(ScalarKind::Int)],
                Box::new(Type::Scalar(ScalarKind::Int)),
            ),
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_parse_call() {
        let source = "myFunc(arg1, arg2)";
        let result = parse(source).unwrap();
        let expected = Expr::new(
            ExprKind::Call(Call::new(
                Identifier::new(String::from("myFunc")),
                vec![
                    Expr::new(
                        ExprKind::Identifier(Identifier::new(String::from("arg1"))),
                        Type::Unknown,
                    ),
                    Expr::new(
                        ExprKind::Identifier(Identifier::new(String::from("arg2"))),
                        Type::Unknown,
                    ),
                ],
            )),
            Type::Unknown,
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_parse_loop() {
        let source = "while (true) { return 42 }";
        let result = parse(source).unwrap();
        let expected = Expr::new(
            ExprKind::Loop(Loop::new(
                Box::new(Expr::new(
                    ExprKind::Literal(LiteralKind::Bool(true)),
                    Type::Scalar(ScalarKind::Bool),
                )),
                Box::new(vec![Expr::new(
                    ExprKind::Return(Return::new(Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(42)),
                        Type::Scalar(ScalarKind::Int),
                    )))),
                    Type::Unknown,
                )]),
            )),
            Type::Unit,
        );
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_parse_conditional() {
        let source = "if (true) { return 42 } else { return 24 }";
        let result = parse(source).unwrap();

        let expected = Expr::new(
            ExprKind::Conditional(Conditional::new(
                Box::new(Expr::new(
                    ExprKind::Literal(LiteralKind::Bool(true)),
                    Type::Scalar(ScalarKind::Bool),
                )),
                Box::new(Expr::new(
                    ExprKind::Return(Return::new(Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(42)),
                        Type::Scalar(ScalarKind::Int),
                    )))),
                    Type::Unknown,
                )),
                Box::new(Expr::new(
                    ExprKind::Return(Return::new(Box::new(Expr::new(
                        ExprKind::Literal(LiteralKind::Int(24)),
                        Type::Scalar(ScalarKind::Int),
                    )))),
                    Type::Unknown,
                )),
            )),
            Type::Unknown,
        );

        assert_eq!(result[0], expected);
    }
}
