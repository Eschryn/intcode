use crate::{ OpSet, Op };
use std::marker::PhantomData;

pub struct IntCode<Os: OpSet>(Vec<u32>, PhantomData<Os>);

impl<Os: OpSet> IntCode<Os> {
    pub fn run(&mut self) -> Result<u32, &'_ str> {
        for pos in (0..self.0.len()).step_by(4) {
            let op = self.read_op(pos)?;

            if !self.exec(op) {
                break;
            }
        }

        Ok(self.0[0])
    }

    fn read_op(&self, pos: usize) -> Result<Op<Os>, &'static str> {
        if self.0.len() - pos >= 4 {
            Ok((&self.0[pos..]).into())
        } else {
            Err("Insufficient parameters")
        }
    }

    pub fn write(&mut self, address: usize, value: u32) {
        if address > self.0.len() {
            self.0.resize(address + 1, 0);
        }

        self.0[address] = value;
    }

    /// Returns if the program should continue
    pub fn exec(&mut self, op: Op<Os>) -> bool {
        op.exec(&mut self.0)
    }
}

impl<Os: OpSet> std::str::FromStr for IntCode<Os> {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            IntCode(
                s
                    .split(',')
                    .map(|n| n.parse::<u32>())
                    .collect::<Result<Vec<u32>, Self::Err>>()?,
                PhantomData
            )
        )
    }
}