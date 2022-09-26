use crate::compiler::vm::{make_op, OpCode};
use crate::{Compile, Node, Operator};

#[derive(Debug, Clone, PartialEq, Eq)]
// ANCHOR: bytecode
pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Node>,
}
// ANCHOR_END: bytecode

impl Bytecode {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}

#[derive(Debug)]
// ANCHOR: bytecode_interpreter
pub struct Interpreter {
    bytecode: Bytecode,
}

impl Compile for Interpreter {
    type Output = Bytecode;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut interpreter = Interpreter {
            bytecode: Bytecode::new(),
        };
        for node in ast {
            println!("compiling node {:?}", node);
            interpreter.interpret_node(node);
            // pop one element from the stack after
            // each expression statement to clean up
            interpreter.add_instruction(OpCode::OpPop);
        }
        interpreter.bytecode
    }
}
// ANCHOR_END: bytecode_interpreter

impl Interpreter {
    fn add_constant(&mut self, node: Node) -> u16 {
        self.bytecode.constants.push(node);
        (self.bytecode.constants.len() - 1) as u16 // cast to u16 because that is the size of our constant pool index
    }

    fn add_instruction(&mut self, op_code: OpCode) -> u16 {
        let position_of_new_instruction = self.bytecode.instructions.len() as u16;
        self.bytecode.instructions.extend(make_op(op_code));
        println!(
            "added instructions {:?} from opcode {:?}",
            self.bytecode.instructions,
            op_code.clone()
        );
        position_of_new_instruction
    }

    fn interpret_node(&mut self, node: Node) {
        match node {
            Node::Int(num) => {
                let const_index = self.add_constant(Node::Int(num));
                self.add_instruction(OpCode::OpConstant(const_index));
            }
            Node::UnaryExpr { op, child } => {
                self.interpret_node(*child);
                match op {
                    Operator::Plus => self.add_instruction(OpCode::OpPlus),
                    Operator::Minus => self.add_instruction(OpCode::OpMinus),
                };
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                self.interpret_node(*lhs);
                self.interpret_node(*rhs);
                match op {
                    Operator::Plus => self.add_instruction(OpCode::OpAdd),
                    Operator::Minus => self.add_instruction(OpCode::OpSub),
                };
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        infix_template("+", OpCode::OpAdd);
        infix_template("-", OpCode::OpSub);
    }

    fn infix_template(infix_str: &str, op_code: OpCode) {
        let input = format!("1 {} 2;", infix_str);
        let bytecode = Interpreter::from_source(&input);

        let expected_instructions = vec![
            OpCode::OpConstant(0),
            OpCode::OpConstant(1),
            op_code,
            OpCode::OpPop,
        ]
        .into_iter()
        .flat_map(make_op)
        .collect();

        assert_eq!(
            Bytecode {
                instructions: expected_instructions,
                constants: vec![Node::Int(1), Node::Int(2)]
            },
            bytecode
        );
    }
}
