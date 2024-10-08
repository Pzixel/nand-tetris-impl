use std::{env, str::FromStr};
use std::io::Write;
use nandtetris_shared::assembler::{self, CodeLine};

#[derive(Debug)]
struct Context {
    label_index: u16,
}

impl Default for Context {
    fn default() -> Self {
        Self { label_index: 1 }
    }
}

impl Context {
    pub fn translate(&mut self, code: &str) -> Vec<String> {
        let instructions = self.parse(code);
        let assembler = instructions.into_iter().flat_map(|x| self.translate_instruction(x));
        let output = assembler.map(|x| x.to_string()).collect::<Vec<_>>();
        output
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
            "sub" => VmInstruction::Sub,
            "neg" => VmInstruction::Neg,
            "eq" => VmInstruction::Eq,
            "gt" => VmInstruction::Gt,
            "lt" => VmInstruction::Lt,
            "and" => VmInstruction::And,
            "or" => VmInstruction::Or,
            "not" => VmInstruction::Not,
            _ => panic!("Invalid command: {}", command),
        }
    }

    fn translate_instruction(&mut self, instruction: VmInstruction) -> Vec<assembler::CodeLine> {
        use assembler::*;

        match instruction {
            VmInstruction::Push { segment, index } => {
                match segment {
                    Segment::Constant => {
                        let mut vec = Vec::with_capacity(7);
                        vec.extend([
                            CodeLine::A(Address::Value(index)),
                            CodeLine::C {
                                dest: Dest::D,
                                comp: Comp::A,
                                jump: Jump::Null,
                            },
                        ]);
                        vec.extend(push(Comp::D));
                        vec
                    }
                }
            }
            VmInstruction::Add => {
                arithmetic(Comp::DPlusA)
            }
            VmInstruction::Sub => {
                arithmetic(Comp::AMinusD)
            }
            VmInstruction::Neg => {
                let mut vec = Vec::with_capacity(7);
                vec.extend(pop(Dest::D));
                vec.push(CodeLine::C {
                    dest: Dest::D,
                    comp: Comp::NegD,
                    jump: Jump::Null,
                });
                vec.extend(push(Comp::D));
                vec
            }
            VmInstruction::Eq => {
                comparison(Jump::JEQ, &mut self.label_index)
            }
            VmInstruction::Gt => {
                comparison(Jump::JGT, &mut self.label_index)
            }
            VmInstruction::Lt => {
                comparison(Jump::JLT, &mut self.label_index)
            }
            VmInstruction::And => {
                arithmetic(Comp::DAndA)
            }
            VmInstruction::Or => {
                arithmetic(Comp::DOrA)
            }
            VmInstruction::Not => {
                let mut vec = Vec::with_capacity(7);
                vec.extend(pop(Dest::D));
                vec.push(CodeLine::C {
                    dest: Dest::D,
                    comp: Comp::NotD,
                    jump: Jump::Null,
                });
                vec.extend(push(Comp::D));
                vec
            }
        }
    }
}

fn arithmetic(comp: assembler::Comp) -> Vec<assembler::CodeLine> {
    use assembler::*;
    let mut vec = Vec::with_capacity(12);
    vec.extend(pop(Dest::D));
    vec.extend(pop(Dest::A));
    vec.push(CodeLine::C {
        dest: Dest::D,
        comp,
        jump: Jump::Null,
    });
    vec.extend(push(Comp::D));
    vec
}

fn push(comp: assembler::Comp) -> [assembler::CodeLine; 5] {
    use assembler::*;
    [
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::A,
            comp: Comp::M,
            jump: Jump::Null,
        },
        CodeLine::C {
            dest: Dest::M,
            comp,
            jump: Jump::Null,
        },
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::M,
            comp: Comp::MPlusOne,
            jump: Jump::Null,
        },
    ]
}

fn pop(dest: assembler::Dest) -> [assembler::CodeLine; 5] {
    use assembler::*;
    [
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::M,
            comp: Comp::MMinusOne,
            jump: Jump::Null,
        },
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::A,
            comp: Comp::M,
            jump: Jump::Null,
        },
        CodeLine::C {
            dest,
            comp: Comp::M,
            jump: Jump::Null,
        },
    ]
}

fn comparison(jump: assembler::Jump, label_index: &mut u16) -> Vec<CodeLine> {
    use assembler::*;

    let mut vec = Vec::with_capacity(25);
    vec.extend(pop(Dest::D));
    vec.extend(pop(Dest::A));
    let label1 = format!("LABEL{}", *label_index);
    let label2 = format!("LABEL{}", *label_index + 1);
    *label_index += 2;
    vec.extend([
        CodeLine::C {
            dest: Dest::D,
            comp: Comp::AMinusD,
            jump: Jump::Null,
        },
        CodeLine::A(Address::Variable(label1.clone().into())),
        CodeLine::C {
            dest: Dest::default(),
            comp: Comp::D,
            jump,
        },
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::A,
            comp: Comp::M,
            jump: Jump::Null,
        },
        CodeLine::C {
            dest: Dest::M,
            comp: Comp::Zero,
            jump: Jump::Null,
        },
        CodeLine::A(Address::Variable(label2.clone().into())),
        CodeLine::C {
            dest: Dest::default(),
            comp: Comp::Zero,
            jump: Jump::JMP,
        },
        CodeLine::Label(label1.into()),
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::A,
            comp: Comp::M,
            jump: Jump::Null,
        },
        CodeLine::C {
            dest: Dest::M,
            comp: Comp::NegOne,
            jump: Jump::Null,
        },
        CodeLine::Label(label2.into()),
        predefined_symbols::SP.into(),
        CodeLine::C {
            dest: Dest::M,
            comp: Comp::MPlusOne,
            jump: Jump::Null,
        },
    ]);
    vec
}

enum VmInstruction {
    Push {
        segment: Segment,
        index: u16,
    },
    Add,
    Eq,
    Lt,
    Gt,
    Sub,
    Neg,
    And,
    Or,
    Not,
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
    use pretty_assertions::assert_eq;

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
            let expected = expected.trim().lines().collect::<Vec<_>>();

            let instructions = Context::default().translate(input);
            let instructions = instructions.iter().collect::<Vec<_>>();

            assert_eq!(instructions, expected);
        };
    }

    #[test]
    fn test_simple_add() {
        test_program!("SimpleAdd");
    }

    #[test]
    fn test_stack_test() {
        test_program!("StackTest");
    }
}
