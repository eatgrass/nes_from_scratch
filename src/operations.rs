#![allow(non_snake_case)]

use crate::cpu::CPU;

pub type Operation = fn(cpu: &mut CPU);

pub fn TAY(cpu: &mut CPU) {
    cpu.iy = cpu.acc;
    if cpu.acc == 0 {
        cpu.status |= 0b00000010;
    }
    if cpu.iy & 0b10000000 > 0 {
        cpu.status |= 0b10000000;
    }
}

/// Copies the current contents of the stack register into the X register and
/// sets the zero and negative flags as appropriate.
pub fn TSX(cpu: &mut CPU) {
    cpu.ix = cpu.sp;
    if cpu.sp == 0 {
        cpu.status |= 0b00000010;
    }
    if cpu.ix & 0b10000000 > 0 {
        cpu.status |= 0b10000000;
    }
}

/// Copies the current contents of the X register into the accumulator and sets
/// the zero and negative flags as appropriate.
pub fn TXA(cpu: &mut CPU) {
    cpu.acc = cpu.ix;
    if cpu.ix == 0 {
        cpu.status |= 0b00000010;
    }
    if cpu.acc & 0b10000000 > 0 {
        cpu.status |= 0b10000000;
    }
}

// Copies the current contents of the X register into the stack register.
pub fn TXS(cpu: &mut CPU) {
    cpu.sp = cpu.ix;
}

/// Copies the current contents of the Y register into the accumulator and sets
/// the zero and negative flags as appropriate.
pub fn TYA(cpu: &mut CPU) {
    cpu.acc = cpu.iy;
    if cpu.iy == 0 {
        cpu.status |= 0b00000010;
    }
    if cpu.acc & 0b10000000 > 0 {
        cpu.status |= 0b10000000;
    }
}
