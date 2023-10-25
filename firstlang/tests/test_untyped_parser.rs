use firstlang::ast::untyped::*;
use firstlang::syntax::untyped_parser::parse;

#[test]
fn test_parse_identifier() {
    let source = "identifier";
    let result = parse(source).unwrap();
    assert_eq!(
        result[0],
        Expr::new(ExprKind::Identifier(Identifier::new(String::from(
            "identifier"
        ))))
    );
}

#[test]
fn test_parse_literal_int() {
    let source = "42";
    let result = parse(source).unwrap();
    assert_eq!(
        result[0],
        Expr::new(ExprKind::Literal(LiteralKind::Int(42)))
    );
}

#[test]
fn test_parse_literal_bool() {
    let source = "true";
    let result = parse(source).unwrap();
    assert_eq!(
        result[0],
        Expr::new(ExprKind::Literal(LiteralKind::Bool(true)))
    );
}

#[test]
fn test_parse_unary_expr() {
    let source = "-42";
    let result = parse(source).unwrap();
    assert_eq!(
        result[0],
        Expr::new(ExprKind::UnaryExpr(UnaryExpr::new(
            UnaryOp::Minus,
            Box::new(Expr::new(ExprKind::Literal(LiteralKind::Int(42))))
        )))
    );
}

#[test]
fn test_parse_binary_expr() {
    let source = "3 + 5";
    let result = parse(source).unwrap();
    assert_eq!(
        result[0],
        Expr::new(ExprKind::BinaryExpr(BinaryExpr::new(
            BinaryOp::Add,
            Box::new(Expr::new(ExprKind::Literal(LiteralKind::Int(3)))),
            Box::new(Expr::new(ExprKind::Literal(LiteralKind::Int(5))))
        )))
    );
}

#[test]
fn test_parse_function() {
    let source = "def myFunc(param1, param2) { return 42 }";
    let result = parse(source).unwrap();
    let expected = Expr::new(ExprKind::Function(Function::new(
        Identifier::new(String::from("myFunc")),
        vec![
            Parameter::new(Identifier::new(String::from("param1"))),
            Parameter::new(Identifier::new(String::from("param2"))),
        ],
        Box::new(Expr::new(ExprKind::Return(Return::new(Box::new(
            Expr::new(ExprKind::Literal(LiteralKind::Int(42))),
        ))))),
    )));
    assert_eq!(result[0], expected);
}

#[test]
fn test_parse_call() {
    let source = "myFunc(arg1, arg2)";
    let result = parse(source).unwrap();
    let expected = Expr::new(ExprKind::Call(Call::new(
        Identifier::new(String::from("myFunc")),
        vec![
            Expr::new(ExprKind::Identifier(Identifier::new(String::from("arg1")))),
            Expr::new(ExprKind::Identifier(Identifier::new(String::from("arg2")))),
        ],
    )));
    assert_eq!(result[0], expected);
}

#[test]
fn test_parse_loop() {
    let source = "while (true) { return 42 }";
    let result = parse(source).unwrap();
    let expected = Expr::new(ExprKind::Loop(Loop::new(
        Box::new(Expr::new(ExprKind::Literal(LiteralKind::Bool(true)))),
        Box::new(Expr::new(ExprKind::Return(Return::new(Box::new(
            Expr::new(ExprKind::Literal(LiteralKind::Int(42))),
        ))))),
    )));
    assert_eq!(result[0], expected);
}

#[test]
fn test_parse_conditional() {
    let source = "if (true) { return 42 } else { return 24 }";
    let result = parse(source).unwrap();

    let expected = Expr::new(ExprKind::Conditional(Conditional::new(
        Box::new(Expr::new(ExprKind::Literal(LiteralKind::Bool(true)))),
        Box::new(Expr::new(ExprKind::Return(Return::new(Box::new(
            Expr::new(ExprKind::Literal(LiteralKind::Int(42))),
        ))))),
        Box::new(Expr::new(ExprKind::Return(Return::new(Box::new(
            Expr::new(ExprKind::Literal(LiteralKind::Int(24))),
        ))))),
    )));

    assert_eq!(result[0], expected);
}
