#![allow(clippy::upper_case_acronyms)]

use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};

use std::result::Result as StdResult;

use crate::ast::untyped as ast;

#[derive(Parser)]
#[grammar = "syntax/grammar_untyped.pest"]
struct UnTypedParser;

pub fn parse(source: &str) -> StdResult<Vec<ast::Expr>, Error<Rule>> {
    let pairs = UnTypedParser::parse(Rule::File, source)?;
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

fn parse_function(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let fn_ident = build_ast_from_expr(pair.next().unwrap())?; // function name Identifier
    let mut params = vec![];
    let mut prev = pair.clone();
    let mut next = pair;
    loop {
        let next = next.next().unwrap();
        if let Rule::Parameter = next.as_rule() {
            params.push(build_ast_from_expr(next)?);
            prev.next().unwrap();
        } else {
            break;
        }
    }

    let params: Vec<ast::Parameter> = params
        .into_iter()
        .map(|e| e.to_parameter().unwrap())
        .collect();
    let body = build_ast_from_expr(prev.next().unwrap())?;
    Ok(ast::Expr::new(ast::ExprKind::Function(ast::Function::new(
        fn_ident.to_identifier().unwrap(),
        params,
        Box::new(body),
    ))))
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

    Ok(ast::Expr::new(ast::ExprKind::Call(ast::Call::new(
        fn_ident.to_identifier().unwrap(),
        args,
    ))))
}

fn parse_loop(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let cond = build_ast_from_expr(pair.next().unwrap())?;
    let body = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(ast::ExprKind::Loop(ast::Loop::new(
        Box::new(cond),
        Box::new(body),
    ))))
}

fn parse_conditional(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let cond = build_ast_from_expr(pair.next().unwrap())?;
    let on_true = build_ast_from_expr(pair.next().unwrap())?;
    let on_false = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(ast::ExprKind::Conditional(
        ast::Conditional::new(Box::new(cond), Box::new(on_true), Box::new(on_false)),
    )))
}

fn parse_assignment(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let ident = build_ast_from_expr(pair.next().unwrap())?;
    let value = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(ast::ExprKind::Assignment(
        ast::Assignment::new(ident.to_identifier().unwrap(), Box::new(value)),
    )))
}

fn parse_return(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let value = build_ast_from_expr(pair.next().unwrap())?;
    Ok(ast::Expr::new(ast::ExprKind::Return(ast::Return::new(
        Box::new(value),
    ))))
}

fn parse_literal(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::Int => {
            let int = pair.as_str().trim().parse::<i64>().unwrap();
            Ok(ast::Expr::new(ast::ExprKind::Literal(
                ast::LiteralKind::Int(int),
            )))
        }
        Rule::Bool => {
            let b = pair.as_str().trim().parse::<bool>().unwrap();
            Ok(ast::Expr::new(ast::ExprKind::Literal(
                ast::LiteralKind::Bool(b),
            )))
        }
        unknown => panic!("Unknown literal: {:?}", unknown),
    }
}

fn parse_identifier(pair: Pair<Rule>) -> StdResult<ast::Expr, Error<Rule>> {
    Ok(ast::Expr::new(ast::ExprKind::Identifier(
        ast::Identifier::new(pair.as_str().to_string()),
    )))
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
        return ast::Expr::new(ast::ExprKind::UnaryExpr(ast::UnaryExpr::new(
            op,
            Box::new(child),
        )));
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
        "<" => ast::BinaryOp::LessThan,
        _ => unreachable!(),
    };
    ast::Expr::new(ast::ExprKind::BinaryExpr(ast::BinaryExpr::new(
        op,
        Box::new(lhs),
        Box::new(rhs),
    )))
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
    Ok(ast::Expr::new(ast::ExprKind::Identifier(
        ident.to_identifier().unwrap(),
    )))
}
