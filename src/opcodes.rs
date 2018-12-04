pub type Opcodes = Vec<u8>;

macro_rules! gen_opcodes {
    ( $( $code:ident: $index:expr ),* ) => {
        $(
            pub const $code: u8 = $index;
        )*
        /// Prints out the oppcode for debugging purposes
        #[allow(dead_code)]
        pub fn print_code(code: u8) -> &'static str {
            match code {
                $(
                    $code => stringify!($code),
                )*
                _ => "UNKNOWN"
            }
        }
    }
}

gen_opcodes!(
    RETURN: 0,
    CALL_FUNCTION: 1,
    JUMP: 2,
    JUMP_ABSOLUTE: 3,
    JUMP_IF_FALSE: 4,

    // Binary Opcodes
    BIN_ADD: 10,
    BIN_SUB: 11,
    BIN_MUL: 12,
    BIN_DIV: 13,
    BIN_EQUAL: 14,
    BIN_NOT_EQUAL: 15,
    BIN_LESS_THAN: 16,
    BIN_LESS_THAN_EQUAL: 17,
    BIN_GREATER_THAN: 18,
    BIN_GREATER_THAN_EQUAL: 19,

    // Misc Opcodes
    LOAD_CONST: 20,

    // Iter Operations
    BUILD_LIST: 25,

    // Loading and Storing
    LOAD_FAST: 30,
    STORE_FAST: 31,
    LOAD_TRUE: 32,
    LOAD_FALSE: 33,
    LOAD_NONE: 34,

    // Unary Operations
    INC_ONE: 40
);
