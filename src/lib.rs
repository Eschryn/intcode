mod int_code;
mod op;

pub use int_code::IntCode;
pub use op::Op;

pub trait OpExec<Os: OpSet> {
    fn exec(op: Op<Os>, execution_target: &mut [u32]) -> bool;
}

pub trait OpSet: 'static + Sized + Eq {
    type OpCode: PartialEq + From<u32>;
    type OpExec: OpExec<Self>;
}