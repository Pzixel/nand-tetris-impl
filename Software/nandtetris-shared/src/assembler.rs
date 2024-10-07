use std::{borrow::Cow, str::FromStr};

#[derive(Debug)]
pub struct Dest {
    pub a: bool,
    pub m: bool,
    pub d: bool,
}

impl Dest {
    pub const A: Dest = Dest { a: true, m: false, d: false };
    pub const M: Dest = Dest { a: false, m: true, d: false };
    pub const D: Dest = Dest { a: false, m: false, d: true };
}

impl From<&Dest> for u16 {
    fn from(value: &Dest) -> u16 {
        u16::from(value.a as u8) << 2 | u16::from(value.d as u8) << 1 | u16::from(value.m as u8)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, derive_more::FromStr)]
pub enum Jump {
    Null = 0b000,
    JGT = 0b001,
    JEQ = 0b010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

impl From<&Jump> for u16 {
    fn from(value: &Jump) -> u16 {
        *value as u16
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum Comp {
    // a=0
    Zero = 0b0101010,
    One = 0b0111111,
    NegOne = 0b0111010,
    D = 0b0001100,
    A = 0b0110000,
    NotD = 0b0001101,
    NotA = 0b0110001,
    NegD = 0b0001111,
    NegA = 0b0110011,
    DPlusOne = 0b0011111,
    APlusOne = 0b0110111,
    DMinusOne = 0b0001110,
    AMinusOne = 0b0110010,
    DPlusA = 0b0000010,
    DMinusA = 0b0010011,
    AMinusD = 0b0000111,
    DAndA = 0b0000000,
    DOrA = 0b0010101,

    // a=1
    M = 0b1110000,
    NotM = 0b1110001,
    NegM = 0b1110011,
    MPlusOne = 0b1110111,
    MMinusOne = 0b1110010,
    DPlusM = 0b1000010,
    DMinusM = 0b1010011,
    MMinusD = 0b1000111,
    DAndM = 0b1000000,
    DOrM = 0b1010101,
}

impl From<&Comp> for u16 {
    fn from(value: &Comp) -> u16 {
        *value as u16
    }
}

impl FromStr for Comp {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::NegOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "-D" => Ok(Comp::NegD),
            "-A" => Ok(Comp::NegA),
            "D+1" => Ok(Comp::DPlusOne),
            "A+1" => Ok(Comp::APlusOne),
            "D-1" => Ok(Comp::DMinusOne),
            "A-1" => Ok(Comp::AMinusOne),
            "D+A" => Ok(Comp::DPlusA),
            "D-A" => Ok(Comp::DMinusA),
            "A-D" => Ok(Comp::AMinusD),
            "D&A" => Ok(Comp::DAndA),
            "D|A" => Ok(Comp::DOrA),
            "M" => Ok(Comp::M),
            "!M" => Ok(Comp::NotM),
            "-M" => Ok(Comp::NegM),
            "M+1" => Ok(Comp::MPlusOne),
            "M-1" => Ok(Comp::MMinusOne),
            "D+M" => Ok(Comp::DPlusM),
            "D-M" => Ok(Comp::DMinusM),
            "M-D" => Ok(Comp::MMinusD),
            "D&M" => Ok(Comp::DAndM),
            "D|M" => Ok(Comp::DOrM),
            _ => Err("Invalid comp")
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Value(u16),
    Variable(Cow<'static, str>),
}

#[derive(Debug)]
pub enum CodeLine {
    Label(String),
    A(Address),
    C {
        comp: Comp,
        dest: Dest,
        jump: Jump,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PredefinedSymbol {
    pub name: &'static str,
    pub value: u16,
}

pub mod predefined_symbols {
    use super::PredefinedSymbol;

    pub const SP: PredefinedSymbol = PredefinedSymbol { name: stringify!(SP), value: 0 };
    pub const LCL: PredefinedSymbol = PredefinedSymbol { name: stringify!(LCL), value: 1 };
    pub const ARG: PredefinedSymbol = PredefinedSymbol { name: stringify!(ARG), value: 2 };
    pub const THIS: PredefinedSymbol = PredefinedSymbol { name: stringify!(THIS), value: 3 };
    pub const THAT: PredefinedSymbol = PredefinedSymbol { name: stringify!(THAT), value: 4 };
    pub const R0: PredefinedSymbol = PredefinedSymbol { name: stringify!(R0), value: 0 };
    pub const R1: PredefinedSymbol = PredefinedSymbol { name: stringify!(R1), value: 1 };
    pub const R2: PredefinedSymbol = PredefinedSymbol { name: stringify!(R2), value: 2 };
    pub const R3: PredefinedSymbol = PredefinedSymbol { name: stringify!(R3), value: 3 };
    pub const R4: PredefinedSymbol = PredefinedSymbol { name: stringify!(R4), value: 4 };
    pub const R5: PredefinedSymbol = PredefinedSymbol { name: stringify!(R5), value: 5 };
    pub const R6: PredefinedSymbol = PredefinedSymbol { name: stringify!(R6), value: 6 };
    pub const R7: PredefinedSymbol = PredefinedSymbol { name: stringify!(R7), value: 7 };
    pub const R8: PredefinedSymbol = PredefinedSymbol { name: stringify!(R8), value: 8 };
    pub const R9: PredefinedSymbol = PredefinedSymbol { name: stringify!(R9), value: 9 };
    pub const R10: PredefinedSymbol = PredefinedSymbol { name: stringify!(R10), value: 10 };
    pub const R11: PredefinedSymbol = PredefinedSymbol { name: stringify!(R11), value: 11 };
    pub const R12: PredefinedSymbol = PredefinedSymbol { name: stringify!(R12), value: 12 };
    pub const R13: PredefinedSymbol = PredefinedSymbol { name: stringify!(R13), value: 13 };
    pub const R14: PredefinedSymbol = PredefinedSymbol { name: stringify!(R14), value: 14 };
    pub const R15: PredefinedSymbol = PredefinedSymbol { name: stringify!(R15), value: 15 };
    pub const SCREEN: PredefinedSymbol = PredefinedSymbol { name: stringify!(SCREEN), value: 16384 };
    pub const KBD: PredefinedSymbol = PredefinedSymbol { name: stringify!(KBD), value: 24576 };
}

pub const PREDEFINED_SYMBOLS: &[PredefinedSymbol] = &[
    predefined_symbols::SP,
    predefined_symbols::LCL,
    predefined_symbols::ARG,
    predefined_symbols::THIS,
    predefined_symbols::THAT,
    predefined_symbols::R0,
    predefined_symbols::R1,
    predefined_symbols::R2,
    predefined_symbols::R3,
    predefined_symbols::R4,
    predefined_symbols::R5,
    predefined_symbols::R6,
    predefined_symbols::R7,
    predefined_symbols::R8,
    predefined_symbols::R9,
    predefined_symbols::R10,
    predefined_symbols::R11,
    predefined_symbols::R12,
    predefined_symbols::R13,
    predefined_symbols::R14,
    predefined_symbols::R15,
    predefined_symbols::SCREEN,
    predefined_symbols::KBD,
];
