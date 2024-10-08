pub mod core;


use std::env;
use std::io::Write;
use core::Context;

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

    #[test]
    fn test_basic_test() {
        test_program!("BasicTest");
    }
}
