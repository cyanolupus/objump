use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() -> Result<(), ObjumpError> {
    let mut opecodemap: HashMap<String, String> = std::collections::HashMap::new();
    for line in io::stdin().lines() {
        match parse_objdump_line(&line?) {
            Ok(ObjDumpLineType::Instruction(instruction)) => match instruction.instruction.opcode {
                X8664ATTOpcode::Unknown(opcode) => {
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
    Insb,
    Imull,
    Subl,
    Js,
    Outsl,
    Xorb,
    Addb,
    Adcb,
    Addl,
    Pushq,
    Andl,
    Cmpb,
    Orl,
    Cmpl,
    Sbbb,
    Orb,
    Adcl,
    Lock,
    Movb,
    Rolb,
    Xchgl,
    Jne,
    Andb,
    Outl,
    Loopne,
    Addq,
    Movabsl,
    Lodsl,
    Lretl,
    Subb,
    Popq,
    Movslq,
    Jae,
    Xorl,
    Jb,
    Outsb,
    Addr32,
    Jo,
    Je,
    Insl,
    Ja,
    Gs,
    Outsw,
    Jns,
    Jbe,
    Imulw,
    Jp,
    Movsl,
    Scasl,
    Int,
    Movl,
    Jl,
    Clc,
    Xlatb,
    Outb,
    Jge,
    Nop,
    Testb,
    Shlb,
    Fsubs,
    Movabsb,
    Shrb,
    Callq,
    Cwtl,
    Fsubrs,
    Rep,
    Hlt,
    Incl,
    Jnp,
    Xorq,
    Fdivs,
    Sbbl,
    Rcrb,
    Rorb,
    Loope,
    Stosl,
    Testl,
    Enter,
    Sarb,
    Jno,
    Fnsave,
    Retq,
    Cmpq,
    Inb,
    Ds,
    Fdivrs,
    Xchgb,
    Fadds,
    Fimull,
    Movw,
    Lodsb,
    Leal,
    Andq,
    Fmuls,
    Jmp,
    Fiadds,
    Cli,
    Cmc,
    Stc,
    Incb,
    Std,
    Repne,
    Sti,
    Fldl,
    Popfq,
    Subq,
    Movq,
    Testq,
    Jmpq,
    Nopl,
    Testw,
    Incq,
    Rolw,
    Andw,
    Tzcntl,
    Leaq,
    Nopw,
    Movzbl,
    Bsrq,
    Notl,
    Shrq,
    Shlq,
    Cmovaeq,
    Movups,
    Movaps,
    Orq,
    Setb,
    Sete,
    Decq,
    Adcq,
    Cmovaq,
    Movabsq,
    Shll,
    Notq,
    Imulq,
    Xorps,
    Pause,
    Cmpxchgl,
    Cmovneq,
    Ud2,
    Decl,
    Jle,
    Sarq,
    Cqto,
    Idivq,
    Shrl,
    Setne,
    Jg,
    Cmovbq,
    Cmoveq,
    Negq,
    Movzwl,
    Sbbq,
    Mulq,
    Setae,
    Movd,
    Punpcklbw,
    Pshuflw,
    Pshufd,
    Movdqa,
    Movdqu,
    Pcmpeqb,
    Pand,
    Pmovmskb,
    Btq,
    Cmpw,
    Cmovgeq,
    Cmovel,
    Cmoval,
    Pinsrw,
    Pxor,
    Movss,
    Movsd,
    Btl,
    Divb,
    Negb,
    Cmovbl,
    Cmovbeq,
    Setbe,
    Movsbq,
    Notb,
    Divq,
    Divl,
    Pextrw,
    Psllw,
    Por,
    Unknown(String),
}

impl From<&str> for X8664ATTOpcode {
    fn from(opcode: &str) -> Self {
        match opcode {
            "add" => X8664ATTOpcode::Add,
            "insb" => X8664ATTOpcode::Insb,
            "imull" => X8664ATTOpcode::Imull,
            "subl" => X8664ATTOpcode::Subl,
            "js" => X8664ATTOpcode::Js,
            "outsl" => X8664ATTOpcode::Outsl,
            "xorb" => X8664ATTOpcode::Xorb,
            "addb" => X8664ATTOpcode::Addb,
            "adcb" => X8664ATTOpcode::Adcb,
            "addl" => X8664ATTOpcode::Addl,
            "pushq" => X8664ATTOpcode::Pushq,
            "andl" => X8664ATTOpcode::Andl,
            "cmpb" => X8664ATTOpcode::Cmpb,
            "orl" => X8664ATTOpcode::Orl,
            "cmpl" => X8664ATTOpcode::Cmpl,
            "sbbb" => X8664ATTOpcode::Sbbb,
            "orb" => X8664ATTOpcode::Orb,
            "adcl" => X8664ATTOpcode::Adcl,
            "lock" => X8664ATTOpcode::Lock,
            "movb" => X8664ATTOpcode::Movb,
            "rolb" => X8664ATTOpcode::Rolb,
            "xchgl" => X8664ATTOpcode::Xchgl,
            "jne" => X8664ATTOpcode::Jne,
            "andb" => X8664ATTOpcode::Andb,
            "outl" => X8664ATTOpcode::Outl,
            "loopne" => X8664ATTOpcode::Loopne,
            "addq" => X8664ATTOpcode::Addq,
            "movabsl" => X8664ATTOpcode::Movabsl,
            "lodsl" => X8664ATTOpcode::Lodsl,
            "lretl" => X8664ATTOpcode::Lretl,
            "subb" => X8664ATTOpcode::Subb,
            "popq" => X8664ATTOpcode::Popq,
            "movslq" => X8664ATTOpcode::Movslq,
            "jae" => X8664ATTOpcode::Jae,
            "xorl" => X8664ATTOpcode::Xorl,
            "jb" => X8664ATTOpcode::Jb,
            "outsb" => X8664ATTOpcode::Outsb,
            "addr32" => X8664ATTOpcode::Addr32,
            "jo" => X8664ATTOpcode::Jo,
            "je" => X8664ATTOpcode::Je,
            "insl" => X8664ATTOpcode::Insl,
            "ja" => X8664ATTOpcode::Ja,
            "gs" => X8664ATTOpcode::Gs,
            "outsw" => X8664ATTOpcode::Outsw,
            "jns" => X8664ATTOpcode::Jns,
            "jbe" => X8664ATTOpcode::Jbe,
            "imulw" => X8664ATTOpcode::Imulw,
            "jp" => X8664ATTOpcode::Jp,
            "movsl" => X8664ATTOpcode::Movsl,
            "scasl" => X8664ATTOpcode::Scasl,
            "int" => X8664ATTOpcode::Int,
            "movl" => X8664ATTOpcode::Movl,
            "jl" => X8664ATTOpcode::Jl,
            "clc" => X8664ATTOpcode::Clc,
            "xlatb" => X8664ATTOpcode::Xlatb,
            "outb" => X8664ATTOpcode::Outb,
            "jge" => X8664ATTOpcode::Jge,
            "nop" => X8664ATTOpcode::Nop,
            "testb" => X8664ATTOpcode::Testb,
            "shlb" => X8664ATTOpcode::Shlb,
            "fsubs" => X8664ATTOpcode::Fsubs,
            "movabsb" => X8664ATTOpcode::Movabsb,
            "shrb" => X8664ATTOpcode::Shrb,
            "callq" => X8664ATTOpcode::Callq,
            "cwtl" => X8664ATTOpcode::Cwtl,
            "fsubrs" => X8664ATTOpcode::Fsubrs,
            "rep" => X8664ATTOpcode::Rep,
            "hlt" => X8664ATTOpcode::Hlt,
            "incl" => X8664ATTOpcode::Incl,
            "jnp" => X8664ATTOpcode::Jnp,
            "xorq" => X8664ATTOpcode::Xorq,
            "fdivs" => X8664ATTOpcode::Fdivs,
            "sbbl" => X8664ATTOpcode::Sbbl,
            "rcrb" => X8664ATTOpcode::Rcrb,
            "rorb" => X8664ATTOpcode::Rorb,
            "loope" => X8664ATTOpcode::Loope,
            "stosl" => X8664ATTOpcode::Stosl,
            "testl" => X8664ATTOpcode::Testl,
            "enter" => X8664ATTOpcode::Enter,
            "sarb" => X8664ATTOpcode::Sarb,
            "jno" => X8664ATTOpcode::Jno,
            "fnsave" => X8664ATTOpcode::Fnsave,
            "retq" => X8664ATTOpcode::Retq,
            "cmpq" => X8664ATTOpcode::Cmpq,
            "inb" => X8664ATTOpcode::Inb,
            "ds" => X8664ATTOpcode::Ds,
            "fdivrs" => X8664ATTOpcode::Fdivrs,
            "xchgb" => X8664ATTOpcode::Xchgb,
            "fadds" => X8664ATTOpcode::Fadds,
            "fimull" => X8664ATTOpcode::Fimull,
            "movw" => X8664ATTOpcode::Movw,
            "lodsb" => X8664ATTOpcode::Lodsb,
            "leal" => X8664ATTOpcode::Leal,
            "andq" => X8664ATTOpcode::Andq,
            "fmuls" => X8664ATTOpcode::Fmuls,
            "jmp" => X8664ATTOpcode::Jmp,
            "fiadds" => X8664ATTOpcode::Fiadds,
            "cli" => X8664ATTOpcode::Cli,
            "cmc" => X8664ATTOpcode::Cmc,
            "stc" => X8664ATTOpcode::Stc,
            "incb" => X8664ATTOpcode::Incb,
            "std" => X8664ATTOpcode::Std,
            "repne" => X8664ATTOpcode::Repne,
            "sti" => X8664ATTOpcode::Sti,
            "fldl" => X8664ATTOpcode::Fldl,
            "popfq" => X8664ATTOpcode::Popfq,
            "subq" => X8664ATTOpcode::Subq,
            "movq" => X8664ATTOpcode::Movq,
            "testq" => X8664ATTOpcode::Testq,
            "jmpq" => X8664ATTOpcode::Jmpq,
            "nopl" => X8664ATTOpcode::Nopl,
            "testw" => X8664ATTOpcode::Testw,
            "incq" => X8664ATTOpcode::Incq,
            "rolw" => X8664ATTOpcode::Rolw,
            "andw" => X8664ATTOpcode::Andw,
            "tzcntl" => X8664ATTOpcode::Tzcntl,
            "leaq" => X8664ATTOpcode::Leaq,
            "nopw" => X8664ATTOpcode::Nopw,
            "movzbl" => X8664ATTOpcode::Movzbl,
            "bsrq" => X8664ATTOpcode::Bsrq,
            "notl" => X8664ATTOpcode::Notl,
            "shrq" => X8664ATTOpcode::Shrq,
            "shlq" => X8664ATTOpcode::Shlq,
            "cmovaeq" => X8664ATTOpcode::Cmovaeq,
            "movups" => X8664ATTOpcode::Movups,
            "movaps" => X8664ATTOpcode::Movaps,
            "orq" => X8664ATTOpcode::Orq,
            "setb" => X8664ATTOpcode::Setb,
            "sete" => X8664ATTOpcode::Sete,
            "decq" => X8664ATTOpcode::Decq,
            "adcq" => X8664ATTOpcode::Adcq,
            "cmovaq" => X8664ATTOpcode::Cmovaq,
            "movabsq" => X8664ATTOpcode::Movabsq,
            "shll" => X8664ATTOpcode::Shll,
            "notq" => X8664ATTOpcode::Notq,
            "imulq" => X8664ATTOpcode::Imulq,
            "xorps" => X8664ATTOpcode::Xorps,
            "pause" => X8664ATTOpcode::Pause,
            "cmpxchgl" => X8664ATTOpcode::Cmpxchgl,
            "cmovneq" => X8664ATTOpcode::Cmovneq,
            "ud2" => X8664ATTOpcode::Ud2,
            "decl" => X8664ATTOpcode::Decl,
            "jle" => X8664ATTOpcode::Jle,
            "sarq" => X8664ATTOpcode::Sarq,
            "cqto" => X8664ATTOpcode::Cqto,
            "idivq" => X8664ATTOpcode::Idivq,
            "shrl" => X8664ATTOpcode::Shrl,
            "setne" => X8664ATTOpcode::Setne,
            "jg" => X8664ATTOpcode::Jg,
            "cmovbq" => X8664ATTOpcode::Cmovbq,
            "cmoveq" => X8664ATTOpcode::Cmoveq,
            "negq" => X8664ATTOpcode::Negq,
            "movzwl" => X8664ATTOpcode::Movzwl,
            "sbbq" => X8664ATTOpcode::Sbbq,
            "mulq" => X8664ATTOpcode::Mulq,
            "setae" => X8664ATTOpcode::Setae,
            "movd" => X8664ATTOpcode::Movd,
            "punpcklbw" => X8664ATTOpcode::Punpcklbw,
            "pshuflw" => X8664ATTOpcode::Pshuflw,
            "pshufd" => X8664ATTOpcode::Pshufd,
            "movdqa" => X8664ATTOpcode::Movdqa,
            "movdqu" => X8664ATTOpcode::Movdqu,
            "pcmpeqb" => X8664ATTOpcode::Pcmpeqb,
            "pand" => X8664ATTOpcode::Pand,
            "pmovmskb" => X8664ATTOpcode::Pmovmskb,
            "btq" => X8664ATTOpcode::Btq,
            "cmpw" => X8664ATTOpcode::Cmpw,
            "cmovgeq" => X8664ATTOpcode::Cmovgeq,
            "cmovel" => X8664ATTOpcode::Cmovel,
            "cmoval" => X8664ATTOpcode::Cmoval,
            "pinsrw" => X8664ATTOpcode::Pinsrw,
            "pxor" => X8664ATTOpcode::Pxor,
            "movss" => X8664ATTOpcode::Movss,
            "movsd" => X8664ATTOpcode::Movsd,
            "btl" => X8664ATTOpcode::Btl,
            "divb" => X8664ATTOpcode::Divb,
            "negb" => X8664ATTOpcode::Negb,
            "cmovbl" => X8664ATTOpcode::Cmovbl,
            "cmovbeq" => X8664ATTOpcode::Cmovbeq,
            "setbe" => X8664ATTOpcode::Setbe,
            "movsbq" => X8664ATTOpcode::Movsbq,
            "notb" => X8664ATTOpcode::Notb,
            "divq" => X8664ATTOpcode::Divq,
            "divl" => X8664ATTOpcode::Divl,
            "pextrw" => X8664ATTOpcode::Pextrw,
            "psllw" => X8664ATTOpcode::Psllw,
            "por" => X8664ATTOpcode::Por,
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

        let offset = parts
            .next()
            .ok_or(ObjumpError::InvalidInstruction(memory.to_string()))?
            .try_into()?;
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

        Ok(X8664ATTAddress {
            displacement,
            base,
            offset,
            scaler,
        })
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
            Ok(X8664ATTValue::Immediate(X8664ATTImmediate::try_from(
                value,
            )?))
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
    Ok(X8664ATTInstruction {
        opcode,
        operands,
        data,
    })
}

fn parse_objdump_line(line: &str) -> Result<ObjDumpLineType, ObjumpError> {
    let line = line.split("#").next().unwrap();

    let parts: Vec<&str> = line.split("\t").collect();
    if parts.len() < 1 || parts[0].is_empty() {
        return Ok(ObjDumpLineType::Blank);
    }

    let data_regex = Regex::new(r"^[0-9a-fA-F]{16}.*")?;
    let instruction_regex = Regex::new(r"^\W*[0-9a-fA-F]{1,16}:.*")?;

    if instruction_regex.is_match(parts[0]) {
        let parts_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let address = u64::from_str_radix(parts_parts[0].trim_end_matches(":"), 16)?;
        let bytes: Vec<u8> = parts_parts[1..]
            .iter()
            .map(|byte| u8::from_str_radix(byte, 16).unwrap())
            .collect();
        let instruction = parse_x8664_att_instruction(parts[1])?;
        return Ok(ObjDumpLineType::Instruction(ObjDumpInstructionLine {
            address,
            bytes,
            instruction,
        }));
    } else if data_regex.is_match(parts[0]) {
        let parts_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let address = u64::from_str_radix(parts_parts[0], 16)?;
        let data = parts_parts[1..].join(" ");
        return Ok(ObjDumpLineType::Data(ObjDumpDataLine { address, data }));
    } else {
        return Ok(ObjDumpLineType::Other(line.to_string()));
    }
}

struct ObjDumpParserIterator<'a> {
    lines: std::str::Lines<'a>,
}

impl<'a> ObjDumpParserIterator<'a> {
    fn new(input: &'a str) -> Self {
        ObjDumpParserIterator {
            lines: input.lines(),
        }
    }
}

impl<'a> Iterator for ObjDumpParserIterator<'a> {
    type Item = Result<ObjDumpLineType, ObjumpError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| parse_objdump_line(line))
    }
}
