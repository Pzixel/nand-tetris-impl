use core::fmt;
use std::borrow::Cow;
use std::{env, fmt::Display};
use std::io::Write;
use nandtetris_shared::assembler::{Address, CodeLine, Comp, Dest, Jump, PREDEFINED_SYMBOLS};

#[derive(Debug)]
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
    symbols: std::collections::HashMap<Cow<'static, str>, u16>,
    next_address: u16,
}

impl SymbolTable {
    fn get_or_insert(&mut self, variable: Cow<'static, str>) -> u16 {
        *self.symbols.entry(variable).or_insert_with(|| {
            let address = self.next_address;
            self.next_address += 1;
            address
        })
    }

    fn insert(&mut self, symbol: Cow<'static, str>, address: u16) {
        // dbg!(&self, &symbol, &address);
        if let Some(x) = self.symbols.insert(symbol.clone(), address) {
            panic!("Symbol {} already exists with address {}", symbol, x);
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable {
            symbols: PREDEFINED_SYMBOLS.iter().map(|x| (x.name.into(), x.value)).collect(),
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
        let mut code_lines = content.lines()
            .map(|line| {
                if let Some(comment_idx) = line.find("//") {
                    line[..comment_idx].trim()
                } else {
                    line.trim()
                }
            })
            .filter(|x| !x.is_empty())
            .map(CodeLine::from_str)

            .collect::<Vec<_>>()
            ;
        let mut line_number = 0;
        for line in code_lines.iter_mut() {
            match line {
                CodeLine::Label(ref mut label) => {
                    self.symbol_table.insert(std::mem::take(label).into(), line_number as u16);
                }
                _ => {
                    line_number += 1;
                }
            }
        }

        code_lines.into_iter().filter_map(|line| {
            match line {
                CodeLine::A(Address::Variable(symbol)) => {
                    let address = self.symbol_table.get_or_insert(symbol);
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
}

fn main() {
    let file_name = env::args().nth(1).expect("No file name provided");
    assert!(file_name.ends_with(".asm"), "File must have .asm extension");
    let file = std::fs::read_to_string(&file_name).expect("Could not read file");
    let instructions = Context::default().assemble(&file);
    let out_file = file_name.replace(".asm", ".hack");
    let file = std::fs::File::create(&out_file).expect("Could not create file");
    let mut writer = std::io::BufWriter::new(file);
    for instruction in instructions {
        writeln!(writer, "{}", instruction).expect("Could not write to file");
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

    #[test]
    fn fill() {
        test_program!("Fill", "");
    }

    #[test]
    fn mult() {
        test_program!("Mult", "");
    }
}
