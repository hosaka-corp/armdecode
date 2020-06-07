//! Functions for decoding instructions.
//!
//! Each function in this module corresponds to an instruction with a unique
//! set of associated bitfields required to decode it.

use crate::*;
use armbf::*;

/// Condition codes.
#[derive(Debug)]
pub enum Cond {
    None, Eq, Ne, Cs, Cc, Mi, Pl, Vs, Vc, Hi, Ls, Ge, Lt, Gt, Le, Al,
}
impl Cond {
    pub fn from_u32(x: u32) -> Self {
        match x { 
            0b0000 => Cond::Eq,
            0b0001 => Cond::Ne,
            0b0010 => Cond::Cs,
            0b0011 => Cond::Cc,
            0b0100 => Cond::Mi,
            0b0101 => Cond::Pl,
            0b0110 => Cond::Vs,
            0b0111 => Cond::Vc,
            0b1000 => Cond::Hi,
            0b1001 => Cond::Ls,
            0b1010 => Cond::Ge,
            0b1011 => Cond::Lt,
            0b1100 => Cond::Gt,
            0b1101 => Cond::Le,
            0b1110 => Cond::Al,
            _ => unreachable!(),
        }
    }
}

/// Data-processing opcodes.
#[derive(Debug)]
pub enum Opcode {
    None, And, Eor, Sub, Rsb, Add, Adc, Sbc, Rsc, Tst, Teq, Cmp, Cmn, Orr, Mov, Bic, Mvn,
}
impl Opcode {
    pub fn from_u32(x: u32) -> Self {
        match x { 
            0b0000 => Opcode::And,
            0b0001 => Opcode::Eor,
            0b0010 => Opcode::Sub,
            0b0011 => Opcode::Rsb,
            0b0100 => Opcode::Add,
            0b0101 => Opcode::Adc,
            0b0110 => Opcode::Sbc,
            0b0111 => Opcode::Rsc,
            0b1000 => Opcode::Tst,
            0b1001 => Opcode::Teq,
            0b1010 => Opcode::Cmp,
            0b1011 => Opcode::Cmn,
            0b1100 => Opcode::Orr,
            0b1101 => Opcode::Mov,
            0b1110 => Opcode::Bic,
            0b1111 => Opcode::Mvn,
            _ => unreachable!(),
        }
    }
}


/// Unique types/kinds of instructions.
#[derive(Debug)]
pub enum ArmInst {
    // Undefined instruction
    None,

    // Control instructions
    Mrs_reg, Mrs_imm, Msr,
    Bx, Blx, Bxj, Clz,
    Qadd, Qsub, QdAdd, QdSub,
    Smla, Smlaw, Smulw, Smlal, Smul,
    Bkpt,

    // Misc. load/store instructions
    Swp, Swpb,
    Ldrh_imm, Ldrh_reg,
    StrdLdrd_imm, StrdLdrd_reg,
    StrhLdrh_reg, StrhLdrh_imm,
    Ldrsh_imm, Ldrsh_reg,
    Ldrsb_imm, Ldrsb_reg,

    // Multiply instructions
    MulMla, UmulUmla,

    // Data-processing instructions
    DpShiftReg, DpShiftImm, DpRotImm,
    
    // Load/store instructions
    LsShift, LsImm, LsMulti,

    // Branching instructions
    Branch,

    // Coprocessor instructions
    CoprocLs, CoprocDp, CoprocRt, 

    // Software interupts
    Swi,
}
impl Default for ArmInst { fn default() -> Self { ArmInst::None } }
