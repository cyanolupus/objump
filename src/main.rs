use std::io;
use regex::Regex;

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        println!("{:?}", parse_objdump_line(&line?));
    }
    Ok(())
}

#[derive(Debug)]
enum ObjDumpLineType {
    Instruction(ObjDumpInstructionLine),
    Data(ObjDumpDataLine),
    Other(String),
    Blank,
}

#[derive(Debug)]
struct ObjDumpInstructionLine {
    address: u64,
    bytes: u64,
    instruction: String,
}

#[derive(Debug)]
struct ObjDumpDataLine {
    address: u64,
    data: String,
}

#[derive(Debug)]
enum ObjumpError {
    ParseError(std::num::ParseIntError),
    RegexError(regex::Error),
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

struct X8664Instruction {
    opcode: X8664Opecode,
    operands: Vec<X8664Operand>,
    data: String,
}

enum X8664Opecode {
    MOV,
    ADD,
    SUB,
    Unknown(String),
}

impl X8664Opecode {
    fn from_str(s: &str) -> Self {
        match s {
            "mov" => X8664Opecode::MOV,
            "add" => X8664Opecode::ADD,
            "sub" => X8664Opecode::SUB,
            _ => X8664Opecode::Unknown(s.to_string()),
        }
    }
}

enum X8664Operand {
    Register(X8664Register),
    Immediate(u64),
    Memory(X8664Memory),
}

enum X8664Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Unknown(String),
}

impl X8664Register {
    fn from_str(s: &str) -> Self {
        match s {
            "rax" => X8664Register::RAX,
            "rbx" => X8664Register::RBX,
            "rcx" => X8664Register::RCX,
            "rdx" => X8664Register::RDX,
            "rsi" => X8664Register::RSI,
            "rdi" => X8664Register::RDI,
            "rsp" => X8664Register::RSP,
            "rbp" => X8664Register::RBP,
            "r8" => X8664Register::R8,
            "r9" => X8664Register::R9,
            "r10" => X8664Register::R10,
            "r11" => X8664Register::R11,
            "r12" => X8664Register::R12,
            "r13" => X8664Register::R13,
            "r14" => X8664Register::R14,
            "r15" => X8664Register::R15,
            _ => X8664Register::Unknown(s.to_string()),
        }
    }
}

struct X8664Memory {
    base: X8664Register,
    index: X8664Register,
    scale: u64,
    displacement: u64,
}

impl X8664Memory {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(|c| c == '(' || c == ')').collect();
        let base = X8664Register::from_str(parts[1]);
        let index = X8664Register::from_str(parts[3]);
        let scale = parts[2].parse().unwrap();
        let displacement = parts[0].parse().unwrap();
        X8664Memory { base, index, scale, displacement }
    }
}

fn parse_objdump_line(line: &str) -> Result<ObjDumpLineType, ObjumpError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 1 {
        return Ok(ObjDumpLineType::Blank);
    }
    let data_regex = Regex::new(r"^[0-9a-fA-F]{16}")?;
    let instruction_regex = Regex::new(r"^[0-9a-fA-F]{8}:")?;

    if data_regex.is_match(parts[0]) {
        let address = u64::from_str_radix(parts[0], 16)?;
        let data = parts[1..].join(" ");
        return Ok(ObjDumpLineType::Data(ObjDumpDataLine { address, data }));
    } else if instruction_regex.is_match(parts[0]) {
        let address = u64::from_str_radix(&parts[0][..8], 16)?;
        let bytes = u64::from_str_radix(&parts[1], 16)?;
        let instruction = parts[2..].join(" ");
        return Ok(ObjDumpLineType::Instruction(ObjDumpInstructionLine { address, bytes, instruction }));
    } else {
        return Ok(ObjDumpLineType::Other(line.to_string()));
    }
}
