use crate::{cpu::CPU, operations::{TYA, Operation}};

pub struct Instruction {
    op_code: u8,
    operate: Operation,
}

impl Instruction {
    pub fn exec(&self, cpu: &mut CPU) {
        (self.operate)(cpu);
    }
}

pub static tya: Instruction = Instruction {
    op_code: 0x98,
    operate: TYA,
};
