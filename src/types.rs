use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    SHA3,
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    BASEFEE,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    LOG0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    pub opcode: Opcode,
    pub stack_input: Vec<[u8; 32]>,
}

impl Operation {
    pub fn new(opcode: Opcode) -> Self {
        Operation {
            opcode,
            stack_input: Vec::new(),
        }
    }

    pub fn with_words(self, num_words: u8, bytes: &mut VecDeque<u8>) -> Self {
        if num_words == 0 {
            return self;
        }
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

    pub fn with_bytes(self, num_bytes: usize, bytes: &mut VecDeque<u8>) -> Self {
        if num_bytes == 0 {
            return self;
        }
        if num_bytes > 32 {
            panic!("Cannot have more than 32 bytes in a word");
        }
        let mut word = [0u8; 32];
        word[32-num_bytes..].clone_from_slice(&bytes.drain(0..num_bytes).collect::<Vec<u8>>());
        Operation {
            opcode: self.opcode,
            stack_input: vec![word],
        }
    }
}


