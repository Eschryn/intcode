use crate::{ OpSet, OpExec };

pub struct Op<Os: OpSet> {
    pub code: Os::OpCode,
    pub inputs: (usize, usize),
    pub output: usize 
}

impl<Os: OpSet> Op<Os> {
    /// Returns if the program should continue
    pub fn exec(self, execution_target: &mut [u32]) -> bool {
        <Os as OpSet>::OpExec::exec(self, execution_target)
    }
}

impl<Os: OpSet> From<&[u32]> for Op<Os> {
    fn from(op: &[u32]) -> Self {
        Self {
            code: Into::into(op[0]),
            inputs: (op[1] as usize, op[2] as usize),
            output: op[3] as usize
        }
    }
}

impl<Os: OpSet> From<u32> for Op<Os> {
    fn from(op: u32) -> Self {
        Self {
            code: Into::into(op),
            inputs: (0, 0),
            output: 0
        }
    }
}