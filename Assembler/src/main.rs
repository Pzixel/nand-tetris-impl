use core::fmt;
use std::{env, fmt::Display};

struct Dest {
    a: bool,
    m: bool,
    d: bool,
}

impl From<&Dest> for u16 {
    fn from(value: &Dest) -> u16 {
        u16::from(value.a as u8) << 2 | u16::from(value.m as u8) << 1 | u16::from(value.d as u8)
    }
}

struct Jump {
    gt: bool,
    eq: bool,
    lt: bool,
}

impl From<&Jump> for u16 {
    fn from(value: &Jump) -> u16 {
        u16::from(value.lt as u8) << 2 | u16::from(value.eq as u8) << 1 | u16::from(value.gt as u8)
    }
}

struct Comp(u16);

impl Comp {
    const ZERO: Comp = Comp(0b0101010);
    const ONE: Comp = Comp(0b0111111);
    const NEG_ONE: Comp = Comp(0b0111010);
    const D: Comp = Comp(0b0001100);
    const A: Comp = Comp(0b0110000);
    const NOT_D: Comp = Comp(0b0001101);
    const NOT_A: Comp = Comp(0b0110001);
    const NEG_D: Comp = Comp(0b0001111);
    const NEG_A: Comp = Comp(0b0110011);
    const D_PLUS_ONE: Comp = Comp(0b0011111);
    const A_PLUS_ONE: Comp = Comp(0b0110111);
    const D_MINUS_ONE: Comp = Comp(0b0001110);
    const A_MINUS_ONE: Comp = Comp(0b0110010);
    const D_PLUS_A: Comp = Comp(0b0000010);
    const D_MINUS_A: Comp = Comp(0b0010011);
    const A_MINUS_D: Comp = Comp(0b0000111);
    const D_AND_A: Comp = Comp(0b0000000);
    const D_OR_A: Comp = Comp(0b0010101);

    const M: Comp = Comp(0b1110000);
    const NOT_M: Comp = Comp(0b1110001);
    const NEG_M: Comp = Comp(0b1110011);
    const M_PLUS_ONE: Comp = Comp(0b1110111);
    const M_MINUS_ONE: Comp = Comp(0b1110010);
    const D_PLUS_M: Comp = Comp(0b1000010);
    const D_MINUS_M: Comp = Comp(0b1010011);
    const M_MINUS_D: Comp = Comp(0b1000111);
    const D_AND_M: Comp = Comp(0b1000000);
    const D_OR_M: Comp = Comp(0b1010101);
}

enum Command {
    A(u16),
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
                instruction = instruction << 7 | comp.0;
                instruction = instruction << 3 | u16::from(dest);
                instruction = instruction << 3 | u16::from(jump);
                Instruction(instruction)
            }
        }
    }
}

fn assemble(content: &str) -> Vec<Instruction> {
    let commands = parse_file(content);
    commands.iter().map(|c| Instruction::from(c)).collect()
}

fn parse_file(content: &str) -> Vec<Command> {
    content.lines()
        .map(|line| {
            if let Some(comment_idx) = line.find("//") {
                line[..comment_idx].trim()
            } else {
                line.trim()
            }
        })
        .filter(|x| !x.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Command {
    if line.as_bytes()[0] == b'@' {
        Command::A(line[1..].parse().expect("Invalid A command"))
    } else {
        let mut dest = Dest { a: false, m: false, d: false };
        let mut jump = Jump { gt: false, eq: false, lt: false };
        let mut comp = &line[..];
        if let Some(idx) = line.find('=') {
            let (d, c) = line.split_at(idx);
            for c in d.chars() {
                match c {
                    'A' => dest.a = true,
                    'M' => dest.m = true,
                    'D' => dest.d = true,
                    _ => panic!("Invalid dest")
                }
            }
            comp = &c[1..];
        }
        if let Some(idx) = comp.find(';') {
            let (c, j) = comp.split_at(idx);
            comp = c;
            for c in j.chars() {
                match c {
                    'G' => jump.gt = true,
                    'E' => jump.eq = true,
                    'L' => jump.lt = true,
                    _ => panic!("Invalid jump")
                }
            }
        }
        let comp = match comp {
            "0" => Comp::ZERO,
            "1" => Comp::ONE,
            "-1" => Comp::NEG_ONE,
            "D" => Comp::D,
            "A" => Comp::A,
            "!D" => Comp::NOT_D,
            "!A" => Comp::NOT_A,
            "-D" => Comp::NEG_D,
            "-A" => Comp::NEG_A,
            "D+1" => Comp::D_PLUS_ONE,
            "A+1" => Comp::A_PLUS_ONE,
            "D-1" => Comp::D_MINUS_ONE,
            "A-1" => Comp::A_MINUS_ONE,
            "D+A" => Comp::D_PLUS_A,
            "D-A" => Comp::D_MINUS_A,
            "A-D" => Comp::A_MINUS_D,
            "D&A" => Comp::D_AND_A,
            "D|A" => Comp::D_OR_A,
            "M" => Comp::M,
            "!M" => Comp::NOT_M,
            "-M" => Comp::NEG_M,
            "M+1" => Comp::M_PLUS_ONE,
            "M-1" => Comp::M_MINUS_ONE,
            "D+M" => Comp::D_PLUS_M,
            "D-M" => Comp::D_MINUS_M,
            "M-D" => Comp::M_MINUS_D,
            "D&M" => Comp::D_AND_M,
            "D|M" => Comp::D_OR_M,
            _ => panic!("Invalid comp")
        };
        Command::C {
            comp,
            dest,
            jump,
        }
    }
}

fn main() {
    let file_name = env::args().nth(1).expect("No file name provided");
    let file = std::fs::read_to_string(file_name).expect("Could not read file");
    let instructions = assemble(&file);
    for instruction in instructions {
        println!("{}", instruction);
    }
}
