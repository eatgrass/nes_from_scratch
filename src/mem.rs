#[derive(Debug)]
pub struct Memory {

}

impl Memory {
    fn translate(self: &Self, mode: AddressingMode) {
    match mode {
        AddressingMode::Immediate => todo!(),
        AddressingMode::ZeroPage => todo!(),
        AddressingMode::ZeroPageX => todo!(),
        AddressingMode::ZeroPageY => todo!(),
        AddressingMode::Absolute => todo!(),
        AddressingMode::AbsoluteX => todo!(),
        AddressingMode::AbsoluteY => todo!(),
        AddressingMode::IndirectX => todo!(),
        AddressingMode::IndirectY => todo!(),
        AddressingMode::NoneAddressing => todo!(),
    }
  }
}

pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}
