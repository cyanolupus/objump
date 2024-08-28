use crate::ObjumpError;
use regex::Regex;

#[derive(Debug)]
pub enum ObjDumpLineType {
    Instruction(ObjDumpInstructionLine),
    Data(ObjDumpDataLine),
    Other(String),
    Blank,
}

pub struct ObjDumpInstructionLine {
    address: u64,
    bytes: Vec<u8>,
    pub instruction: crate::objdump::x8664_att::X8664ATTInstruction,
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

pub fn parse_objdump_line(line: &str) -> Result<ObjDumpLineType, ObjumpError> {
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
        let instruction = crate::objdump::x8664_att::parse_x8664_att_instruction(parts[1])?;
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