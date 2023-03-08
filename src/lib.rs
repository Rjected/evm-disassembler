use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    DIV,
    SDIV,
    MOD,
    SMOD,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    opcode: Opcode,
    stack_input: Vec<[u8; 32]>,
}

impl Operation {
    pub fn new(opcode: Opcode) -> Self {
        Operation {
            opcode,
            stack_input: Vec::new(),
        }
    }

    pub fn with_stack_input(&self, num_words: u8, bytes: &mut VecDeque<u8>) -> Self {
        let stack_input = (0..num_words)
            .map(|_| {
                let mut word = [0u8; 32];
                word.copy_from_slice(&bytes.drain(0..32).collect::<Vec<u8>>());
                word
            })
            .collect();
        Operation {
            opcode: self.opcode,
            stack_input,
        }
    }
}

pub fn disassemble(mut input: &str) -> Vec<Operation> {
    input = input.trim_start_matches("0x");
    let mut bytes = VecDeque::from(hex::decode(input).expect("Invalid hex string"));
    let mut operations = Vec::new();
    while bytes.len() > 0 {
        operations.push(decode_operation(&mut bytes));
    }
    return operations;
}

fn decode_operation(bytes: &mut VecDeque<u8>) -> Operation {
    match bytes.pop_front() {
        None => panic!("Unexpected end of input"),
        Some(value) => match value {
            0x00 => Operation::new(Opcode::STOP),
            0x01 => Operation::new(Opcode::ADD).with_stack_input(2, bytes),
            0x02 => Operation::new(Opcode::MUL).with_stack_input(2, bytes),
            0x03 => Operation::new(Opcode::DIV).with_stack_input(2, bytes),
            0x04 => Operation::new(Opcode::SDIV).with_stack_input(2, bytes),
            0x05 => Operation::new(Opcode::MOD).with_stack_input(2, bytes),
            0x06 => Operation::new(Opcode::SMOD).with_stack_input(2, bytes),
            _ => panic!("Invalid opcode: {}", value),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn pad_word(input: &str) -> [u8; 32] {
        let mut word = [0u8; 32];
        let padded_string = format!("{:0>64}", input);
        hex::decode_to_slice(padded_string, &mut word).expect("Invalid hex string");
        return word;
    }

    fn encode_op(opcode: &str, stack_input: Vec<[u8; 32]>) -> String {
        let mut bytes: String = opcode.to_owned();
        for word in stack_input {
            bytes += &hex::encode(word);
        }
        return bytes;
    }

    #[rstest]
    #[case(Opcode::STOP, "0x00", vec![])]
    #[case(Opcode::ADD, "0x01", vec!["100", "1234567"])]
    #[case(Opcode::MUL, "0x02", vec!["100", "1234567"])]
    #[case(Opcode::DIV, "0x03", vec!["100", "1234567"])]
    #[case(Opcode::SDIV, "0x04", vec!["100", "1234567"])]
    #[case(Opcode::MOD, "0x05", vec!["100", "1234567"])]
    #[case(Opcode::SMOD, "0x06", vec!["100", "1234567"])]
    fn decode_single_op(#[case] opcode: Opcode, #[case] encoded_opcode: &str, #[case] arguments: Vec<&str>) {
        let stack_input: Vec<[u8;32]> = arguments.iter().map(|arg| pad_word(arg)).collect();
        println!("stack_input: {:?}", stack_input);
        let encoded_op = encode_op(encoded_opcode, stack_input.clone());
        let result = disassemble(&encoded_op);
        assert_eq!(
            result,
            vec![Operation {
                opcode,
                stack_input,
            }]
        );
    }

    #[rstest]
    fn decode_stop_and_add() {
        let a = pad_word("100");
        let b = pad_word("1234567");
        let add_op = encode_op("01", vec![a, b]);
        let stop_op = encode_op("00", vec![]);
        let result = disassemble(&(add_op + &stop_op));
        assert_eq!(
            result,
            vec![Operation {
                opcode: Opcode::ADD,
                stack_input: vec![a, b],
            },
            Operation {
                opcode: Opcode::STOP,
                stack_input: vec![],
            }]
        );
    }
}
