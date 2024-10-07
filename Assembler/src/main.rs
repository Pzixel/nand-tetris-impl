use core::fmt;
use std::{env, fmt::Display, str::FromStr};

#[derive(Debug)]
struct Dest {
    a: bool,
    m: bool,
    d: bool,
}

impl From<&Dest> for u16 {
    fn from(value: &Dest) -> u16 {
        u16::from(value.a as u8) << 2 | u16::from(value.d as u8) << 1 | u16::from(value.m as u8)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, derive_more::FromStr)]
enum Jump {
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
enum Comp {
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
enum Command {
    A(u16),
    C {
        comp: Comp,
        dest: Dest,
        jump: Jump,
    }
}

#[derive(Debug)]
enum Address {
    Value(u16),
    Variable(String),
}

#[derive(Debug)]
enum CodeLine {
    Label(String),
    A(Address),
    C {
        comp: Comp,
        dest: Dest,
        jump: Jump,
    }
}

struct Instruction(u16);

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

impl From<&Command> for Instruction {
    fn from(value: &Command) -> Self {
        match value {
            Command::A(val) => Instruction(*val),
            Command::C { comp, dest, jump } => {
                let mut instruction = 0b111;
                instruction = instruction << 7 | u16::from(comp);
                instruction = instruction << 3 | u16::from(dest);
                instruction = instruction << 3 | u16::from(jump);
                Instruction(instruction)
            }
        }
    }
}

#[derive(Debug)]
struct SymbolTable {
    symbols: std::collections::HashMap<String, u16>,
    next_address: u16,
}

impl SymbolTable {
    fn get_or_insert(&mut self, variable: String) -> u16 {
        *self.symbols.entry(variable).or_insert_with(|| {
            let address = self.next_address;
            self.next_address += 1;
            address
        })
    }

    fn insert(&mut self, symbol: String, address: u16) {
        // dbg!(&self, &symbol, &address);
        if let Some(x) = self.symbols.insert(symbol.clone(), address) {
            panic!("Symbol {} already exists with address {}", symbol, x);
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        let mut symbols = std::collections::HashMap::new();
        symbols.insert("SP".to_string(), 0);
        symbols.insert("LCL".to_string(), 1);
        symbols.insert("ARG".to_string(), 2);
        symbols.insert("THIS".to_string(), 3);
        symbols.insert("THAT".to_string(), 4);
        symbols.insert("R0".to_string(), 0);
        symbols.insert("R1".to_string(), 1);
        symbols.insert("R2".to_string(), 2);
        symbols.insert("R3".to_string(), 3);
        symbols.insert("R4".to_string(), 4);
        symbols.insert("R5".to_string(), 5);
        symbols.insert("R6".to_string(), 6);
        symbols.insert("R7".to_string(), 7);
        symbols.insert("R8".to_string(), 8);
        symbols.insert("R9".to_string(), 9);
        symbols.insert("R10".to_string(), 10);
        symbols.insert("R11".to_string(), 11);
        symbols.insert("R12".to_string(), 12);
        symbols.insert("R13".to_string(), 13);
        symbols.insert("R14".to_string(), 14);
        symbols.insert("R15".to_string(), 15);
        symbols.insert("SCREEN".to_string(), 0x4000);
        symbols.insert("KBD".to_string(), 0x6000);
        SymbolTable {
            symbols,
            next_address: 16,
        }
    }
}

#[derive(Debug, Default)]
struct Context {
    symbol_table: SymbolTable,
}

impl Context {
    fn assemble(&mut self, content: &str) -> Vec<Instruction> {
        let commands = self.parse_file(content);
        // dbg!(&commands);
        commands.iter().map(|c| Instruction::from(c)).collect()
    }

    fn parse_file(&mut self, content: &str) -> Vec<Command> {
        let code_lines = content.lines()
            .map(|line| {
                if let Some(comment_idx) = line.find("//") {
                    line[..comment_idx].trim()
                } else {
                    line.trim()
                }
            })
            .filter(|x| !x.is_empty())
            .map(|x| Self::parse_line(x))

            .collect::<Vec<_>>()
            ;
        let mut line_number = 0;
        for line in code_lines.iter() {
            match line {
                CodeLine::Label(label) => {
                    self.symbol_table.insert(label.clone(), line_number as u16);
                }
                _ => {
                    line_number += 1;
                }
            }
        }

        code_lines.into_iter().filter_map(|line| {
            match line {
                CodeLine::A(Address::Variable(symbol)) => {
                    let address = self.symbol_table.get_or_insert(symbol.clone());
                    Some(Command::A(address))
                }
                CodeLine::A(Address::Value(address)) => {
                    Some(Command::A(address))
                }
                CodeLine::C { comp, dest, jump } => {
                    Some(Command::C { comp, dest, jump })
                }
                _ => None
            }
        }).collect()

    }

    fn parse_line(line: &str) -> CodeLine {
        if line.as_bytes()[0] == b'(' {
            CodeLine::Label(line[1..line.len() - 1].to_string())
        } else if line.as_bytes()[0] == b'@' {
            let value = line[1..].parse().map(Address::Value)
                .unwrap_or_else(|_| Address::Variable(line[1..].to_string()));
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

fn main() {
    let file_name = env::args().nth(1).expect("No file name provided");
    let file = std::fs::read_to_string(file_name).expect("Could not read file");
    let instructions = Context::default().assemble(&file);
    for instruction in instructions {
        println!("{}", instruction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_test_files {
        ($name:literal, $l:literal) => {
            (
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    concat!("/assets/", $name, $l, ".asm")
                )),
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    concat!("/assets/", $name, ".hack")
                ))
            )
        };
    }

    macro_rules! test_program {
        ($name:literal, $l:literal) => {
            let (input, expected) = get_test_files!($name, $l);
            let expected = expected.lines().collect::<Vec<_>>();

            let instructions = Context::default().assemble(input);
            let instructions = instructions.iter().map(|x| x.to_string()).collect::<Vec<_>>();
            assert_eq!(instructions, expected);
        };
    }

    #[test]
    fn add() {
        test_program!("Add", "");
    }

    #[test]
    fn max_l() {
        test_program!("Max", "L");
    }

    #[test]
    fn rect_l() {
        test_program!("Rect", "L");
    }

    #[test]
    fn pong_l() {
        test_program!("Pong", "L");
    }

    #[test]
    fn max() {
        test_program!("Max", "");
    }

    #[test]
    fn rect() {
        test_program!("Rect", "");
    }

    #[test]
    fn pong() {
        test_program!("Pong", "");
    }
}
