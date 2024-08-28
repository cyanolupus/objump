use std::{
    collections::HashMap,
    io::{self, Write},
};

mod objdump;

fn main() -> Result<(), ObjumpError> {
    let mut opecodemap: HashMap<String, String> = std::collections::HashMap::new();
    for line in io::stdin().lines() {
        match objdump::line::parse_objdump_line(&line?) {
            Ok(objdump::line::ObjDumpLineType::Instruction(instruction)) => match instruction.instruction.opcode {
                objdump::x8664_att::X8664ATTOpcode::Unknown(opcode) => {
                    if opecodemap.insert(opcode.clone(), capitalize(&opcode)) == None {
                        println!(
                            "\t\"{}\" => X8664ATTOpcode::{},",
                            opcode,
                            capitalize(&opcode)
                        );
                        eprintln!("\t{},", capitalize(&opcode));
                        io::stdout().flush().unwrap();
                        io::stderr().flush().unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Debug)]
enum ObjumpError {
    ParseError(std::num::ParseIntError),
    RegexError(regex::Error),
    InvalidInstruction(String),
    IOError(io::Error),
}

impl From<regex::Error> for ObjumpError {
    fn from(err: regex::Error) -> Self {
        ObjumpError::RegexError(err)
    }
}

impl From<std::num::ParseIntError> for ObjumpError {
    fn from(err: std::num::ParseIntError) -> Self {
        ObjumpError::ParseError(err)
    }
}

impl From<io::Error> for ObjumpError {
    fn from(err: io::Error) -> Self {
        ObjumpError::IOError(err)
    }
}
