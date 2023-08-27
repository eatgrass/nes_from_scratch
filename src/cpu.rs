use crate::instructions::{tya, Instruction};

/// processor status
/// ┌─┬─┬──┬─┬─┬─┬─┐
/// │N│V│ss│D│I│Z│C│
/// └─┴─┴──┴─┴─┴─┴─┘
#[derive(Debug)]
pub struct CPU {
    pub status: u8,
    pub ix: u8,
    pub iy: u8,
    pub acc: u8,
    pub sp: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            status: 0,
            ix: 0,
            iy: 0,
            acc: 0,
            sp: 0,
            program_counter: 0,
        }
    }
}

impl CPU {
    pub fn exec(&mut self, ins: &Instruction) {
        ins.exec(self)
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_exec() {
    let mut cpu: CPU = CPU::new();
    cpu.iy = 0b01;
    cpu.exec(&tya);
    print!("{:?}", cpu);
}
