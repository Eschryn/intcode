use intcode::{ IntCode, OpExec, OpSet, Op };

#[derive(PartialEq)]
enum OpCode {
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

struct OpExecuter;
impl OpExec<DefaultOpSet> for OpExecuter {
    fn exec(op: Op<DefaultOpSet>, execution_target: &mut [u32]) -> bool {
        match op {
            Op { code: OpCode::Add, inputs, output } => execution_target[output] = execution_target[inputs.0] + execution_target[inputs.1],
            Op { code: OpCode::Mul, inputs, output } => execution_target[output] = execution_target[inputs.0] * execution_target[inputs.1],
            Op { code: OpCode::Brk, .. } => return false,
        }

        true
    }
}

#[derive(PartialEq, Eq)]
struct DefaultOpSet;
impl OpSet for DefaultOpSet {
    type OpExec = OpExecuter;
    type OpCode = OpCode;
}

fn main() {
    let input = std::fs::read_to_string("examples/gravity_assist.ic").unwrap();
    let mut code = input.parse::<IntCode<DefaultOpSet>>().unwrap();
    let res = code.run().unwrap();

    println!("{}", res);
}