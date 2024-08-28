use std::io;
use regex::Regex;

fn main() -> Result<(), ObjumpError> {
    for line in io::stdin().lines() {
        println!("{:?}", parse_objdump_line(&line?)?);
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

struct ObjDumpInstructionLine {
    address: u64,
    bytes: Vec<u8>,
    instruction: X8664ATTInstruction,
}

impl std::fmt::Debug for ObjDumpInstructionLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjDumpInstructionLine")
            .field("address", &format!("{:#x}", self.address))
            .field("bytes", &format!("{:?}", self.bytes))
            .field("instruction", &self.instruction)
            .finish()
    }
}

struct ObjDumpDataLine {
    address: u64,
    data: String,
}

impl std::fmt::Debug for ObjDumpDataLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjDumpDataLine")
            .field("address", &format!("{:#x}", self.address))
            .field("data", &self.data)
            .finish()
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

#[derive(Debug)]
struct X8664ATTInstruction {
    opcode: X8664ATTOpcode,
    operands: Vec<X8664ATTOperand>,
    data: String,
}

#[derive(Debug)]
enum X8664ATTOpcode {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Mov,
    Push,
    Pop,
    Jmp,
    Jne,
    Jl,
    Jle,
    Jg,
    Jge,
    Call,
    Ret,
    Nop,
    Unknown(String),
}

impl From<&str> for X8664ATTOpcode {
    fn from(opcode: &str) -> Self {
        match opcode {
            "add" => X8664ATTOpcode::Add,
            "sub" => X8664ATTOpcode::Sub,
            "and" => X8664ATTOpcode::And,
            "or" => X8664ATTOpcode::Or,
            "xor" => X8664ATTOpcode::Xor,
            "mov" => X8664ATTOpcode::Mov,
            "push" => X8664ATTOpcode::Push,
            "pop" => X8664ATTOpcode::Pop,
            "jmp" => X8664ATTOpcode::Jmp,
            "jne" => X8664ATTOpcode::Jne,
            "jl" => X8664ATTOpcode::Jl,
            "jle" => X8664ATTOpcode::Jle,
            "jg" => X8664ATTOpcode::Jg,
            "jge" => X8664ATTOpcode::Jge,
            "call" => X8664ATTOpcode::Call,
            "ret" => X8664ATTOpcode::Ret,
            "nop" => X8664ATTOpcode::Nop,
            _ => X8664ATTOpcode::Unknown(opcode.to_string()),
        }
    }
}

#[derive(Debug)]
enum X8664ATTOperand {
    Register(X8664ATTRegister),
    Immediate(X8664ATTImmediate),
    Address(X8664ATTAddress),
}

impl TryFrom<&str> for X8664ATTOperand {
    type Error = ObjumpError;

    fn try_from(operand: &str) -> Result<Self, ObjumpError> {
        if operand.starts_with("%") {
            Ok(X8664ATTOperand::Register(operand.into()))
        } else if operand.starts_with("$") {
            Ok(X8664ATTOperand::Immediate(operand.try_into()?))
        } else {
            Ok(X8664ATTOperand::Address(operand.try_into()?))
        }
    }
}

#[derive(Debug)]
enum X8664ATTRegister {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
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

impl From<&str> for X8664ATTRegister {
    fn from(register: &str) -> Self {
        match register {
            "%rax" => X8664ATTRegister::Rax,
            "%rbx" => X8664ATTRegister::Rbx,
            "%rcx" => X8664ATTRegister::Rcx,
            "%rdx" => X8664ATTRegister::Rdx,
            "%rsi" => X8664ATTRegister::Rsi,
            "%rdi" => X8664ATTRegister::Rdi,
            "%rbp" => X8664ATTRegister::Rbp,
            "%rsp" => X8664ATTRegister::Rsp,
            "%r8" => X8664ATTRegister::R8,
            "%r9" => X8664ATTRegister::R9,
            "%r10" => X8664ATTRegister::R10,
            "%r11" => X8664ATTRegister::R11,
            "%r12" => X8664ATTRegister::R12,
            "%r13" => X8664ATTRegister::R13,
            "%r14" => X8664ATTRegister::R14,
            "%r15" => X8664ATTRegister::R15,
            _ => X8664ATTRegister::Unknown(register.to_string()),
        }
    }
}

#[derive(Debug)]
struct X8664ATTAddress {
    displacement: Option<X8664ATTInteger>,
    base: Option<X8664ATTValue>,
    offset: X8664ATTValue,
    scaler: Option<X8664ATTInteger>,
}

impl TryFrom<&str> for X8664ATTAddress {
    type Error = ObjumpError;

    fn try_from(memory: &str) -> Result<Self, ObjumpError> {
        let mut parts = memory.split("(");
        let displacement = match parts.next() {
            Some(displacement) => {
                if displacement.is_empty() {
                    None
                } else {
                    Some(X8664ATTInteger::try_from(displacement)?)
                }
            }
            None => None,
        };

        let mut memory = parts.next().unwrap();
        memory = memory.trim_end_matches(")");
        let mut parts = memory.split(",");
        let base = match parts.next() {
            Some(base) => {
                if base.is_empty() {
                    None
                } else {
                    Some(X8664ATTValue::try_from(base)?)
                }
            }
            None => None,
        };

        let offset = parts.next().ok_or(ObjumpError::InvalidInstruction(memory.to_string()))?.try_into()?;
        let scaler = match parts.next() {
            Some(scaler) => {
                if scaler.is_empty() {
                    None
                } else {
                    Some(X8664ATTInteger::try_from(scaler)?)
                }
            }
            None => None,
        };

        Ok(X8664ATTAddress { displacement, base, offset, scaler })
    }
}

#[derive(Debug)]
enum X8664ATTValue {
    Register(X8664ATTRegister),
    Immediate(X8664ATTImmediate),
}

impl TryFrom<&str> for X8664ATTValue {
    type Error = ObjumpError;

    fn try_from(value: &str) -> Result<Self, ObjumpError> {
        if value.starts_with("$") {
            Ok(X8664ATTValue::Immediate(X8664ATTImmediate::try_from(value)?))
        } else {
            Ok(X8664ATTValue::Register(X8664ATTRegister::from(value)))
        }
    }
}

#[derive(Debug)]
struct X8664ATTImmediate(u64);

impl TryFrom<&str> for X8664ATTImmediate {
    type Error = ObjumpError;

    fn try_from(immediate: &str) -> Result<Self, ObjumpError> {
        if immediate.starts_with("$0x") {
            Ok(X8664ATTImmediate(u64::from_str_radix(&immediate[3..], 16)?))
        } else if immediate.starts_with("$0b") {
            Ok(X8664ATTImmediate(u64::from_str_radix(&immediate[3..], 2)?))
        } else {
            Ok(X8664ATTImmediate(u64::from_str_radix(&immediate[1..], 10)?))
        }
    }
}

#[derive(Debug)]
struct X8664ATTInteger(u64);

impl TryFrom<&str> for X8664ATTInteger {
    type Error = ObjumpError;

    fn try_from(integer: &str) -> Result<Self, ObjumpError> {
        if integer.starts_with("0x") {
            Ok(X8664ATTInteger(u64::from_str_radix(&integer[2..], 16)?))
        } else if integer.starts_with("0b") {
            Ok(X8664ATTInteger(u64::from_str_radix(&integer[2..], 2)?))
        } else {
            Ok(X8664ATTInteger(u64::from_str_radix(integer, 10)?))
        }
    }
}

impl Into<u64> for X8664ATTInteger {
    fn into(self) -> u64 {
        self.0
    }
}

fn parse_x8664_att_instruction(line: &str) -> Result<X8664ATTInstruction, ObjumpError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let opcode = X8664ATTOpcode::from(parts[0]);
    let mut operands = Vec::new();
    let mut data = String::new();
    for part in &parts[1..] {
        if part.starts_with("<") {
            data = part.to_string();
            break;
        } else {
            operands.push(X8664ATTOperand::try_from(*part)?);
        }
    }
    Ok(X8664ATTInstruction { opcode, operands, data })
}

fn parse_objdump_line(line: &str) -> Result<ObjDumpLineType, ObjumpError> {
    let line = line.split("#").next().unwrap();

    let parts: Vec<&str> = line.split("\t").collect();
    if parts.len() < 1 || parts[0].is_empty() {
        return Ok(ObjDumpLineType::Blank);
    }
    println!("{:?}", parts);

    let data_regex = Regex::new(r"^[0-9a-fA-F]{16}.*")?;
    let instruction_regex = Regex::new(r"^\W*[0-9a-fA-F]{1,16}:.*")?;

    if instruction_regex.is_match(parts[0]) {
        let parts_parts: Vec<&str> = parts[0].split_whitespace().collect();
        println!("{:?}", parts_parts);
        let address = u64::from_str_radix(parts_parts[0].trim_end_matches(":"), 16)?;
        let bytes: Vec<u8> = parts_parts[1..].iter().map(|byte| u8::from_str_radix(byte, 16).unwrap()).collect();
        let instruction = parse_x8664_att_instruction(parts[1])?;
        return Ok(ObjDumpLineType::Instruction(ObjDumpInstructionLine { address, bytes, instruction }));
    } else if data_regex.is_match(parts[0]) {
        let parts_parts: Vec<&str> = parts[0].split_whitespace().collect();
        println!("{:?}", parts_parts);
        let address = u64::from_str_radix(parts_parts[0], 16)?;
        let data = parts_parts[1..].join(" ");
        return Ok(ObjDumpLineType::Data(ObjDumpDataLine { address, data }));
    } else {
        return Ok(ObjDumpLineType::Other(line.to_string()));
    }
}
