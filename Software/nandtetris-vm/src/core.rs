use std::str::FromStr;
use nandtetris_shared::assembler::{self, CodeLine};

#[derive(Debug)]
pub struct Context {
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

    fn parse(&self, code: &str) -> Vec<VmInstruction> {
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
                            CodeLine::constant(index),
                            CodeLine::assign(Dest::D, Comp::A),
                        ]);
                        vec.extend(push(Comp::D));
                        vec
                    }
                }
            }
            VmInstruction::Add => {
                binary(Comp::DPlusA)
            }
            VmInstruction::Sub => {
                binary(Comp::AMinusD)
            }
            VmInstruction::Neg => {
                unary(Comp::NegD)
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
                binary(Comp::DAndA)
            }
            VmInstruction::Or => {
                binary(Comp::DOrA)
            }
            VmInstruction::Not => {
                unary(Comp::NotD)
            }
        }
    }
}

fn unary(comp: assembler::Comp) -> Vec<assembler::CodeLine> {
    use assembler::*;
    let mut vec = Vec::with_capacity(7);
    vec.extend(pop(Dest::D));
    vec.push(CodeLine::assign(Dest::D, comp));
    vec.extend(push(Comp::D));
    vec
}

fn binary(comp: assembler::Comp) -> Vec<assembler::CodeLine> {
    use assembler::*;
    let mut vec = Vec::with_capacity(12);
    vec.extend(pop(Dest::D));
    vec.extend(pop(Dest::A));
    vec.push(CodeLine::assign(Dest::D, comp));
    vec.extend(push(Comp::D));
    vec
}

fn push(comp: assembler::Comp) -> [assembler::CodeLine; 5] {
    use assembler::*;
    [
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::A, Comp::M),
        CodeLine::assign(Dest::M, comp),
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::M, Comp::MPlusOne),
    ]
}

fn pop(dest: assembler::Dest) -> [assembler::CodeLine; 5] {
    use assembler::*;
    [
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::M, Comp::MMinusOne),
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::A, Comp::M),
        CodeLine::assign(dest, Comp::M),
    ]
}

fn comparison(jump: assembler::Jump, label_index: &mut u16) -> Vec<CodeLine> {
    use assembler::*;

    let mut vec = Vec::with_capacity(25);
    let label1 = format!("LABEL{}", *label_index);
    let label2 = format!("LABEL{}", *label_index + 1);
    *label_index += 2;


    vec.extend(pop(Dest::D));
    vec.extend(pop(Dest::A));
    vec.extend([
        CodeLine::assign(Dest::D, Comp::AMinusD),
        CodeLine::variable(label1.clone()),
        CodeLine::test(Dest::default(), Comp::D, jump),
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::A, Comp::M),
        CodeLine::assign(Dest::M, Comp::Zero),
        CodeLine::variable(label2.clone()),
        CodeLine::goto(),
        CodeLine::Label(label1.into()),
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::A, Comp::M),
        CodeLine::assign(Dest::M, Comp::NegOne),
        CodeLine::Label(label2.into()),
        predefined_symbols::SP.into(),
        CodeLine::assign(Dest::M, Comp::MPlusOne),
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
