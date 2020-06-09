//! Traits implemented on newtypes for representing bitfields.
//!
//! These traits describe some set of bitfields that may or may not belong to
//! a particular group/class/type of ARM instructions. All of the functions in 
//! these traits have some corresponding macro in the armbf_prim crate which 
//! perform some operation on a u32.

/// Accessors common to all instructions.
pub trait InstBits {
    fn cond(&self) -> u32;
    fn group(&self) -> u32;
}

/// Accessors common to data processing instructions.
pub trait DpBits {
    fn opcd(&self) -> u32;
    fn s(&self) -> bool;
}

/// Accessors common to load/store multiple instructions.
pub trait LsMultiBits {
    fn s(&self) -> bool;
    fn reglist(&self) -> u32;
}

/// Accessors common to load/store instructions.
pub trait LsBits {
    fn p(&self) -> bool;
    fn u(&self) -> bool;
    fn b(&self) -> bool;
    fn w(&self) -> bool;
    fn l(&self) -> bool;
}

/// Accessors for immediates.
pub trait ImmBits {
    fn imm8(&self) -> u32;
    fn imm12(&self) -> u32;
    fn imm24(&self) -> u32;
}

/// Accessors for instructions that modify the status registers.
pub trait SrBits {
    fn field_mask(&self) -> u32;
}

/// Accessors in branching instructions.
pub trait BranchBits { fn link(&self) -> bool; }

/// Accessors for rotate instructions.
pub trait RotBits { fn rot_imm(&self) -> u32; }

/// Accessors for shifter instructions.
pub trait ShiftBits {
    fn shift_imm(&self) -> u32;
    fn shift(&self) -> u32;
}

/// Accessors common to coprocessor instructions.
pub trait CoprocBits {
    fn opcd1(&self) -> u32;
    fn opcd1_rt(&self) -> u32;
    fn cp_num(&self) -> u32;
    fn opcd2(&self) -> u32;
    fn crn(&self) -> u32;
    fn crd(&self) -> u32;
    fn crm(&self) -> u32;
}

/// Accessors for common register fields.
pub trait RegBits {
    fn rn(&self) -> u32;
    fn rd(&self) -> u32;
    fn rm(&self) -> u32;
    fn rs(&self) -> u32;
}


