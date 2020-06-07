#[derive(Debug, Copy, Clone)]
// ANCHOR: vm_opcode
pub enum OpCode {
    OpConstant(u16), // pointer to constant table
    OpPop,           // pop is needed for execution
    OpAdd,
    OpSub,
    OpPlus,
    OpMinus,
}
// ANCHOR_END: vm_opcode

fn convert_u16_to_two_u8s(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert_two_u8s_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_u16_to_two_u8s(data));
    output
}

pub fn make_op(op: OpCode) -> Vec<u8> {
    match op {
        // ANCHOR: vm_make_op
        OpCode::OpConstant(arg) => make_three_byte_op(0x01, arg),
        OpCode::OpPop => vec![0x02],  // decimal repr is 2
        OpCode::OpAdd => vec![0x03],  // decimal repr is 3
        OpCode::OpSub => vec![0x04],  // decimal repr is 4
        OpCode::OpPlus => vec![0x0A], // decimal repr is 10
        OpCode::OpMinus => vec![0x0B], // decimal repr is 11
                                       // ANCHOR_END: vm_make_op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_op_constant() {
        assert_eq!(vec![0x01, 255, 254], make_op(OpCode::OpConstant(65534)));
    }

    #[test]
    fn make_op_pop() {
        assert_eq!(vec![0x02], make_op(OpCode::OpPop));
    }

    #[test]
    fn make_op_add() {
        assert_eq!(vec![0x03], make_op(OpCode::OpAdd));
    }
}
