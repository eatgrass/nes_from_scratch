use crate::cpu::CPU;

pub struct Instruction<'a> {
    op_code: u8,
    operate: &'a Operation,
}

impl Instruction<'_> {
    pub fn exec(&self, cpu: &mut CPU) {
        (self.operate)(cpu);
    }
}

