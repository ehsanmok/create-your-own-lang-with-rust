#![allow(clippy::upper_case_acronyms)]

use pest::{self, Parser};

use crate::ast::{Node, Operator};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;
// ANCHOR_END: parser

// ANCHOR: parse_source
pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}
// ANCHOR_END: parse_source

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(op, child)
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let mut op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if pair_buf != None {
                    op = pair_buf.unwrap();
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(op, lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Int => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[1..]),
                _ => (1, istr),
            };
            let int: i32 = istr.parse().unwrap();
            Node::Int(sign * int)
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node {
    Node::UnaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        assert!(parse("b").is_err());
    }

    #[test]
    fn unary_expr() {
        let plus_one = parse("+1");
        assert!(plus_one.is_ok());
        assert_eq!(
            plus_one.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1))
            }]
        );
        assert_eq!(format!("{}", plus_one.unwrap()[0]), "+1");

        let neg_two = parse("-2");
        assert!(neg_two.is_ok());
        assert_eq!(
            neg_two.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Operator::Minus,
                child: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", neg_two.unwrap()[0]), "-2");
    }
    #[test]
    fn binary_expr() {
        let sum = parse("1 + 2");
        assert!(sum.is_ok());
        assert_eq!(
            sum.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::Int(1)),
                rhs: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", sum.unwrap()[0]), "1 + 2");
        let minus = parse("1   -  \t  2");
        assert!(minus.is_ok());
        assert_eq!(
            minus.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Minus,
                lhs: Box::new(Node::Int(1)),
                rhs: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", minus.unwrap()[0]), "1 - 2");
        // fails as there's no rhs:
        // let paran_sum = parse("(1 + 2)");
        // assert!(paran_sum.is_ok());
    }

    #[test]
    fn nested_expr() {
        fn test_expr(expected: &str, src: &str) {
            assert_eq!(
                expected,
                parse(src)
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &format!("{}", &arg))
            );
        }

        test_expr("1 + 2 + 3", "(1 + 2) + 3");
        test_expr("1 + 2 + 3", "1 + (2 + 3)");
        test_expr("1 + 2 + 3 + 4", "1 + (2 + (3 + 4))");
        test_expr("1 + 2 + 3 - 4", "(1 + 2) + (3 - 4)");
    }

    #[test]
    fn multiple_operators() {
        assert_eq!(
            parse("1+2+3").unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::Plus,
                    lhs: Box::new(Node::Int(1)),
                    rhs: Box::new(Node::Int(2)),
                }),
                rhs: Box::new(Node::Int(3)),
            }]
        )
    }
}
