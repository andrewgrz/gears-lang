pub type Opcodes = Vec<u8>;

// Control Flow Opcodes
pub const RETURN: u8 = 0;
pub const CALL_FUNCTION: u8 = 1;

// Binary Opcodes
pub const BIN_ADD: u8 = 4;
pub const BIN_SUB: u8 = 5;
pub const BIN_MUL: u8 = 6;
pub const BIN_DIV: u8 = 7;

// Misc Opcodes
pub const LOAD_CONST: u8 = 20;

// Loading and Storing
pub const LOAD_FAST: u8 = 30;
pub const STORE_FAST: u8 = 31;
