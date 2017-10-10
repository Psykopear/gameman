use csv;
use std::collections::HashMap;
use std::fs::File;

use cpu::ByteStream;

#[derive(Debug,Deserialize)]
pub struct Operation {
    pub code: String,
    pub mnemonic: String,
    pub into: String,
    pub operand1: Option<String>,
    pub operand2: Option<String>,
    pub bytes: u8,
    pub flag_z: Option<char>,
    pub flag_h: Option<char>,
    pub flag_n: Option<char>,
    pub flag_c: Option<char>,
    pub cycles_ok: u8,
    pub cycles_no: Option<u8>
}

impl Operation {
    pub fn code_as_u8(&self) -> u8 {
        u8::from_str_radix(&self.code[2..], 16)
            .expect(&format!("Opcode is not a number! {}, op", self.code))
    }
}

pub struct Ops {
    ops: HashMap<u8, Operation>,
    cb_ops: HashMap<u8, Operation>
}

impl Ops {
    pub fn new() -> Ops {
        let mut ops = Ops { ops: HashMap::new(), cb_ops: HashMap::new() };
        ops.load_ops();
        ops
    }

    pub fn load_ops(&mut self) {
        Ops::load_op_type(&mut self.ops, "data/unprefixed.csv");
        Ops::load_op_type(&mut self.cb_ops, "data/cbprefixed.csv");
    }

    pub fn load_op_type(map: &mut HashMap<u8, Operation>, filepath: &str) {
        let file = File::open(filepath).expect(&format!("File not found: {}", filepath));

        for result in csv::Reader::from_reader(file).deserialize() {
            let op: Operation = result.expect(&format!("Opcodes CSV file is broken! {}", filepath));
            map.insert(op.code_as_u8(), op);
        }
    }

    pub fn fetch_operation(&mut self, ih: &mut ByteStream) -> &Operation {
        let byte = ih.read_byte();
        let op = self.ops.get(&byte).expect(&format!("Missing operation {:x}! WTF?", byte));
        if op.code_as_u8() == 0xcb {
            let cb_byte = ih.read_byte();
            return self.cb_ops.get(&cb_byte).expect(&format!("Missing operation {:x}! WTF?", cb_byte));
        }
        op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_ops() {
        let ops = Ops::new();

        assert_eq!(ops.ops.get(&0x3e).unwrap().mnemonic, "LD")
    }
}