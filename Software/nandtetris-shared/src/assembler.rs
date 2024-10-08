use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Clone, Copy)]
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

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.a {
            write!(f, "A")?;
        }
        if self.m {
            write!(f, "M")?;
        }
        if self.d {
            write!(f, "D")?;
        }
        if self.a || self.m || self.d {
            write!(f, "=")?;
        }
        Ok(())
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

impl std::fmt::Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Jump::Null => "",
            Jump::JGT => concat!(";", stringify!(JGT)),
            Jump::JEQ => concat!(";", stringify!(JEQ)),
            Jump::JGE => concat!(";", stringify!(JGE)),
            Jump::JLT => concat!(";", stringify!(JLT)),
            Jump::JNE => concat!(";", stringify!(JNE)),
            Jump::JLE => concat!(";", stringify!(JLE)),
            Jump::JMP => concat!(";", stringify!(JMP)),
        };
        write!(f, "{}", s)
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

mod comp_string_repr {
    // declare public &str constants for each Comp variant
    pub const ZERO: &str = "0";
    pub const ONE: &str = "1";
    pub const NEG_ONE: &str = "-1";
    pub const D: &str = "D";
    pub const A: &str = "A";
    pub const NOT_D: &str = "!D";
    pub const NOT_A: &str = "!A";
    pub const NEG_D: &str = "-D";
    pub const NEG_A: &str = "-A";
    pub const D_PLUS_ONE: &str = "D+1";
    pub const A_PLUS_ONE: &str = "A+1";
    pub const D_MINUS_ONE: &str = "D-1";
    pub const A_MINUS_ONE: &str = "A-1";
    pub const D_PLUS_A: &str = "D+A";
    pub const D_MINUS_A: &str = "D-A";
    pub const A_MINUS_D: &str = "A-D";
    pub const D_AND_A: &str = "D&A";
    pub const D_OR_A: &str = "D|A";
    pub const M: &str = "M";
    pub const NOT_M: &str = "!M";
    pub const NEG_M: &str = "-M";
    pub const M_PLUS_ONE: &str = "M+1";
    pub const M_MINUS_ONE: &str = "M-1";
    pub const D_PLUS_M: &str = "D+M";
    pub const D_MINUS_M: &str = "D-M";
    pub const M_MINUS_D: &str = "M-D";
    pub const D_AND_M: &str = "D&M";
    pub const D_OR_M: &str = "D|M";
}

impl FromStr for Comp {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use comp_string_repr::*;

        match s {
            ZERO => Ok(Comp::Zero),
            ONE => Ok(Comp::One),
            NEG_ONE => Ok(Comp::NegOne),
            D => Ok(Comp::D),
            A => Ok(Comp::A),
            NOT_D => Ok(Comp::NotD),
            NOT_A => Ok(Comp::NotA),
            NEG_D => Ok(Comp::NegD),
            NEG_A => Ok(Comp::NegA),
            D_PLUS_ONE => Ok(Comp::DPlusOne),
            A_PLUS_ONE => Ok(Comp::APlusOne),
            D_MINUS_ONE => Ok(Comp::DMinusOne),
            A_MINUS_ONE => Ok(Comp::AMinusOne),
            D_PLUS_A => Ok(Comp::DPlusA),
            D_MINUS_A => Ok(Comp::DMinusA),
            A_MINUS_D => Ok(Comp::AMinusD),
            D_AND_A => Ok(Comp::DAndA),
            D_OR_A => Ok(Comp::DOrA),
            M => Ok(Comp::M),
            NOT_M => Ok(Comp::NotM),
            NEG_M => Ok(Comp::NegM),
            M_PLUS_ONE => Ok(Comp::MPlusOne),
            M_MINUS_ONE => Ok(Comp::MMinusOne),
            D_PLUS_M => Ok(Comp::DPlusM),
            D_MINUS_M => Ok(Comp::DMinusM),
            M_MINUS_D => Ok(Comp::MMinusD),
            D_AND_M => Ok(Comp::DAndM),
            D_OR_M => Ok(Comp::DOrM),
            _ => Err("Invalid Comp string"),
        }
    }
}

impl From<&Comp> for &'static str {
    fn from(value: &Comp) -> &'static str {
        use comp_string_repr::*;

        match value {
            Comp::Zero => ZERO,
            Comp::One => ONE,
            Comp::NegOne => NEG_ONE,
            Comp::D => D,
            Comp::A => A,
            Comp::NotD => NOT_D,
            Comp::NotA => NOT_A,
            Comp::NegD => NEG_D,
            Comp::NegA => NEG_A,
            Comp::DPlusOne => D_PLUS_ONE,
            Comp::APlusOne => A_PLUS_ONE,
            Comp::DMinusOne => D_MINUS_ONE,
            Comp::AMinusOne => A_MINUS_ONE,
            Comp::DPlusA => D_PLUS_A,
            Comp::DMinusA => D_MINUS_A,
            Comp::AMinusD => A_MINUS_D,
            Comp::DAndA => D_AND_A,
            Comp::DOrA => D_OR_A,
            Comp::M => M,
            Comp::NotM => NOT_M,
            Comp::NegM => NEG_M,
            Comp::MPlusOne => M_PLUS_ONE,
            Comp::MMinusOne => M_MINUS_ONE,
            Comp::DPlusM => D_PLUS_M,
            Comp::DMinusM => D_MINUS_M,
            Comp::MMinusD => M_MINUS_D,
            Comp::DAndM => D_AND_M,
            Comp::DOrM => D_OR_M,
        }
    }
}

impl Comp {
    pub fn as_str(&self) -> &'static str {
        self.into()
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

impl CodeLine {
    pub fn from_str(line: &str) -> Self {
        if line.as_bytes()[0] == b'(' {
            CodeLine::Label(line[1..line.len() - 1].to_string())
        } else if line.as_bytes()[0] == b'@' {
            let value = line[1..].parse().map(Address::Value)
                .unwrap_or_else(|_| Address::Variable(line[1..].to_string().into()));
            CodeLine::A(value)
        } else {
            let mut dest = Dest { a: false, m: false, d: false };
            let mut jump = Jump::Null;
            let mut comp = &line[..];
            if let Some(idx) = line.find('=') {
                let (d, c) = line.split_at(idx);
                for c in d.chars() {
                    match c {
                        'A' => dest.a = true,
                        'M' => dest.m = true,
                        'D' => dest.d = true,
                        _ => panic!("Invalid dest {}", d)
                    }
                }
                comp = &c[1..];
            }
            if let Some(idx) = comp.find(';') {
                let (c, j) = comp.split_at(idx);
                comp = c;
                jump = j[1..].parse().unwrap_or_else(|e| panic!("Invalid jump {}: {:?}", j, e));
            }
            let comp = comp.parse().unwrap_or_else(|e| panic!("Invalid comp {}: {:?}", comp, e));
            CodeLine::C {
                comp,
                dest,
                jump,
            }
        }
    }
}

impl std::fmt::Display for CodeLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CodeLine::Label(label) => write!(f, "({})", label),
            CodeLine::A(Address::Value(value)) => write!(f, "@{}", value),
            CodeLine::A(Address::Variable(symbol)) => write!(f, "@{}", symbol),
            CodeLine::C { comp, dest, jump } => {
                write!(f, "{}{}{}",
                    dest,
                    comp.as_str(),
                    jump
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PredefinedSymbol {
    pub name: &'static str,
    pub value: u16,
}

impl From<PredefinedSymbol> for CodeLine {
    fn from(value: PredefinedSymbol) -> CodeLine {
        CodeLine::A(Address::Variable(value.name.into()))
    }
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
