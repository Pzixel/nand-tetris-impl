use core::fmt;
use std::{env, fmt::Display, str::FromStr};
use std::io::Write;

#[derive(Debug, Default)]
struct Context {
}

impl Context {
    pub fn translate(&self, code: &str) -> Vec<String> {
        todo!()
    }

    pub fn parse(&self, code: &str) -> Vec<VmInstruction> {
        let code_lines = code.lines()
            .map(|line| {
                if let Some(comment_idx) = line.find("//") {
                    line[..comment_idx].trim()
                } else {
                    line.trim()
                }
            })
            .filter(|x| !x.is_empty())
            .map(|x| Self::parse_line(x))

            .collect::<Vec<_>>();
        code_lines
    }

    fn parse_line(line: &str) -> VmInstruction {
        let mut parts = line.split_whitespace();
        let command = parts.next().expect("No command found");
        match command {
            "push" => {
                let segment = parts.next().expect("No segment found");
                let index = parts.next().expect("No index found");
                let segment = Segment::from_str(segment).expect("Invalid segment");
                let index = index.parse().expect("Invalid index");
                VmInstruction::Push { segment, index }
            }
            "add" => VmInstruction::Add,
            _ => panic!("Invalid command {}", command),
        }
    }
}

enum VmInstruction {
    Push {
        segment: Segment,
        index: u16,
    },
    Add,
}

impl Display for VmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmInstruction::Push { segment, index } => match segment {
                Segment::Constant => translate_push_constant(*index, f),
            },
            VmInstruction::Add => translate_add(f),
        }
    }
}

fn translate_push_constant(index: u16, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    writeln!(f, "@{}", index)?;
    writeln!(f, "D=A")?;
    writeln!(f, "@SP")?;
    writeln!(f, "A=M")?;
    writeln!(f, "M=D")?;
    writeln!(f, "@SP")?;
    writeln!(f, "M=M+1")?;
    Ok(())
}

fn translate_add(f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    todo!()
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, derive_more::FromStr)]
enum Segment {
    Constant,
}

fn main() {
    let file_name = env::args().nth(1).expect("No file name provided");
    assert!(file_name.ends_with(".vm"), "File name must end with .vm");
    let file = std::fs::read_to_string(&file_name).expect("Could not read file");
    let instructions = Context::default().translate(&file);
    let out_file = file_name.replace(".vm", ".asm");
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
        ($name:literal) => {
            (
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    concat!("/assets/", $name, ".vm")
                )),
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    concat!("/assets/", $name, ".asm")
                ))
            )
        };
    }

    macro_rules! test_program {
        ($name:literal) => {
            let (input, expected) = get_test_files!($name);
            let expected = expected.lines().collect::<Vec<_>>();

            let instructions = Context::default().translate(input);
            let instructions = instructions.iter().map(|x| x.to_string()).collect::<Vec<_>>();
            assert_eq!(instructions, expected);
        };
    }

    #[test]
    fn test_push_constant() {
        test_program!("SimpleAdd");
    }
}
