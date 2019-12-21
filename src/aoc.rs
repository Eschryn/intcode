use crate::{ IntCode, OpExec, OpSet, Op };

#[derive(PartialEq)]
pub enum OpCode {
    Add,
    Mul,
    Brk
}

impl From<u32> for OpCode {
    fn from(a: u32) -> Self {
        match a {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Brk,
            _ => panic!("Unkown opcode!")
        }
    }
}

pub struct OpExecuter;
impl OpExec<AocOpSet> for OpExecuter {
    fn exec(op: Op<AocOpSet>, execution_target: &mut [u32]) -> bool {
        match op {
            Op { code: OpCode::Add, inputs, output } => execution_target[output] = execution_target[inputs.0] + execution_target[inputs.1],
            Op { code: OpCode::Mul, inputs, output } => execution_target[output] = execution_target[inputs.0] * execution_target[inputs.1],
            Op { code: OpCode::Brk, .. } => return false,
        }

        true
    }
}

#[derive(PartialEq, Eq)]
pub struct AocOpSet;
impl OpSet for AocOpSet {
    type OpExec = OpExecuter;
    type OpCode = OpCode;
}

pub type AocIntCode = IntCode<AocOpSet>;