//! Parser for Thirdlang
//!
//! Converts source code into a typed AST using the pest parser generator.
//! Extends Secondlang parser with class definitions, method calls, and field access.

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{
    AssignTarget, BinaryOp, ClassDef, Expr, FieldDef, MethodDef, Program, Stmt, TopLevel,
    TypedExpr, UnaryOp,
};
use crate::types::Type;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ThirdlangParser;

/// Parse source code into a program (list of top-level items)
pub fn parse(source: &str) -> Result<Program, String> {
    let pairs =
        ThirdlangParser::parse(Rule::Program, source).map_err(|e| format!("Parse error: {}", e))?;

    let mut program = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::TopLevel => {
                program.push(parse_top_level(pair)?);
            }
            Rule::EOI => {}
            _ => {}
        }
    }
    Ok(program)
}

fn parse_top_level(pair: Pair<Rule>) -> Result<TopLevel, String> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::ClassDef => Ok(TopLevel::Class(parse_class_def(inner)?)),
        Rule::Stmt => Ok(TopLevel::Stmt(parse_stmt(inner)?)),
        r => Err(format!("Unexpected top-level rule: {:?}", r)),
    }
}

// =============================================================================
// Class Parsing
// =============================================================================

// ANCHOR: parse_class
fn parse_class_def(pair: Pair<Rule>) -> Result<ClassDef, String> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let body = inner.next().unwrap(); // ClassBody

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    for item in body.into_inner() {
        match item.as_rule() {
            Rule::FieldDef => {
                fields.push(parse_field_def(item)?);
            }
            Rule::MethodDef => {
                methods.push(parse_method_def(item)?);
            }
            _ => {}
        }
    }

    Ok(ClassDef {
        name,
        fields,
        methods,
    })
}

fn parse_field_def(pair: Pair<Rule>) -> Result<FieldDef, String> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let ty = parse_type(inner.next().unwrap())?;
    Ok(FieldDef { name, ty })
}

fn parse_method_def(pair: Pair<Rule>) -> Result<MethodDef, String> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    // Skip 'self' parameter
    inner.next(); // SelfParam

    let mut params: Vec<(String, Type)> = Vec::new();
    let mut return_type = Type::Unit;
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

    Ok(MethodDef {
        name,
        params,
        return_type,
        body,
    })
}
// ANCHOR_END: parse_class

// =============================================================================
// Statement Parsing
// =============================================================================

fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt, String> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::Function => parse_function(inner),
        Rule::Delete => parse_delete(inner),
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
        Rule::ClassType => {
            let name = pair.into_inner().next().unwrap().as_str().to_string();
            Ok(Type::Class(name))
        }
        Rule::Identifier => Ok(Type::Class(pair.as_str().to_string())),
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

fn parse_delete(pair: Pair<Rule>) -> Result<Stmt, String> {
    let expr = pair.into_inner().next().unwrap();
    Ok(Stmt::Delete(parse_expr(expr)?))
}

fn parse_assignment(pair: Pair<Rule>) -> Result<Stmt, String> {
    let mut inner = pair.into_inner();
    let target_pair = inner.next().unwrap();

    let target = match target_pair.as_rule() {
        Rule::AssignTarget => {
            let target_inner = target_pair.into_inner().next().unwrap();
            match target_inner.as_rule() {
                Rule::FieldAccess => parse_assign_field_access(target_inner)?,
                Rule::Identifier => AssignTarget::Var(target_inner.as_str().to_string()),
                r => return Err(format!("Unexpected assign target rule: {:?}", r)),
            }
        }
        Rule::FieldAccess => parse_assign_field_access(target_pair)?,
        Rule::Identifier => AssignTarget::Var(target_pair.as_str().to_string()),
        r => return Err(format!("Unexpected target rule: {:?}", r)),
    };

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
        target,
        type_ann,
        value,
    })
}

fn parse_assign_field_access(pair: Pair<Rule>) -> Result<AssignTarget, String> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    let mut object = match first.as_rule() {
        Rule::SelfKeyword => TypedExpr::unknown(Expr::SelfRef),
        Rule::Identifier => TypedExpr::unknown(Expr::Var(first.as_str().to_string())),
        r => return Err(format!("Unexpected field access base: {:?}", r)),
    };

    let mut last_field = String::new();
    for field_pair in inner {
        if !last_field.is_empty() {
            object = TypedExpr::unknown(Expr::FieldAccess {
                object: Box::new(object),
                field: last_field,
            });
        }
        last_field = field_pair.as_str().to_string();
    }

    Ok(AssignTarget::Field {
        object: Box::new(object),
        field: last_field,
    })
}

// =============================================================================
// Expression Parsing
// =============================================================================

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
        Rule::Postfix => return parse_postfix(pair),
        Rule::NewExpr => parse_new_expr(pair)?,
        Rule::FunctionCall => return parse_function_call(pair),
        Rule::Literal => parse_literal(pair)?,
        Rule::Int => Expr::Int(pair.as_str().parse().unwrap()),
        Rule::Bool => Expr::Bool(pair.as_str() == "true"),
        Rule::SelfKeyword => Expr::SelfRef,
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

fn parse_postfix(pair: Pair<Rule>) -> Result<TypedExpr, String> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    let mut expr = parse_expr(first)?;

    // Apply postfix operators (field access, method calls)
    for postfix in inner {
        // PostfixOp contains either MethodCall or FieldAccessOp
        let op = if postfix.as_rule() == Rule::PostfixOp {
            postfix.into_inner().next().unwrap()
        } else {
            postfix
        };

        match op.as_rule() {
            Rule::MethodCall => {
                let mut method_inner = op.into_inner();
                let method = method_inner.next().unwrap().as_str().to_string();
                let args: Vec<TypedExpr> = method_inner
                    .map(|p| parse_expr(p))
                    .collect::<Result<_, _>>()?;

                expr = TypedExpr::unknown(Expr::MethodCall {
                    object: Box::new(expr),
                    method,
                    args,
                });
            }
            Rule::FieldAccessOp => {
                let field = op.into_inner().next().unwrap().as_str().to_string();
                expr = TypedExpr::unknown(Expr::FieldAccess {
                    object: Box::new(expr),
                    field,
                });
            }
            _ => {}
        }
    }

    Ok(expr)
}

fn parse_new_expr(pair: Pair<Rule>) -> Result<Expr, String> {
    let mut inner = pair.into_inner();
    let class = inner.next().unwrap().as_str().to_string();
    let args: Vec<TypedExpr> = inner.map(|p| parse_expr(p)).collect::<Result<_, _>>()?;

    Ok(Expr::New { class, args })
}

fn parse_function_call(pair: Pair<Rule>) -> Result<TypedExpr, String> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let args: Vec<TypedExpr> = inner.map(|p| parse_expr(p)).collect::<Result<_, _>>()?;

    Ok(TypedExpr::unknown(Expr::Call { name, args }))
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
    fn test_parse_class_def() {
        let source = r#"
            class Point {
                x: int
                y: int

                def __init__(self, x: int, y: int) {
                    self.x = x
                    self.y = y
                }

                def get_x(self) -> int {
                    return self.x
                }
            }
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.len(), 1);

        if let TopLevel::Class(class) = &program[0] {
            assert_eq!(class.name, "Point");
            assert_eq!(class.fields.len(), 2);
            assert_eq!(class.methods.len(), 2);
        } else {
            panic!("Expected class definition");
        }
    }

    #[test]
    fn test_parse_new_expr() {
        let source = r#"
            class Point { x: int }
            new Point(42)
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.len(), 2);

        if let TopLevel::Stmt(Stmt::Expr(expr)) = &program[1] {
            if let Expr::New { class, args } = &expr.expr {
                assert_eq!(class, "Point");
                assert_eq!(args.len(), 1);
            } else {
                panic!("Expected new expression");
            }
        } else {
            panic!("Expected statement");
        }
    }

    #[test]
    fn test_parse_method_call() {
        let source = r#"
            class Point { x: int def get_x(self) -> int { return self.x } }
            p = new Point(10)
            p.get_x()
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.len(), 3);
    }

    #[test]
    fn test_parse_delete() {
        let source = r#"
            class Point { x: int }
            p = new Point(42)
            delete p
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.len(), 3);

        if let TopLevel::Stmt(Stmt::Delete(_)) = &program[2] {
            // OK
        } else {
            panic!("Expected delete statement");
        }
    }

    #[test]
    fn test_parse_field_assignment() {
        let source = r#"
            class Point { x: int def set_x(self, val: int) { self.x = val } }
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.len(), 1);
    }
}
