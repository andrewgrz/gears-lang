
pub type Opcodes = Vec<u8>;

// Control Flow Opcodes
pub const RETURN: u8 = 0;

// Binary Opcodes
pub const BIN_ADD: u8 = 4;
pub const BIN_SUB: u8 = 5;
pub const BIN_MUL: u8 = 6;
pub const BIN_DIV: u8 = 7;

// Misc Opcodes
pub const LOAD_CONST: u8 = 20;
