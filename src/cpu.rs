use crate::instructions::Instruction;

// ┌─┬─┬──┬─┬─┬─┬─┐
// │N│V│ss│D│I│Z│C│
// └─┴─┴──┴─┴─┴─┴─┘
#[derive(Debug)]
pub struct CPU {
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            status: 0,
            program_counter: 0,
        }
    }
}

impl CPU {
    pub fn exec(&mut self, ins: Instruction) {
        ins.exec(self)
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
