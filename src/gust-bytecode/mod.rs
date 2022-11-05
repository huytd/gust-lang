#[repr(i32)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum OpCode {
    PUSH,
    GLOAD,
    GSTORE,
    LLOAD,
    LSTORE,
    CALL,
    RET,
    ADD,
    SUB,
    MUL,
    DIV,
    PRINT,
    HALT,
    POP,
    // Comparison ops
    EQ,
    NE,
    GT,
    LT,
    GE,
    LE,
    // Jumping
    JMP,
    JMP0,
    JMP1
}

impl From<i32> for OpCode {
    fn from(n: i32) -> Self {
        unsafe { std::mem::transmute(n) }
    }
}

pub const FUNC_PARAM_OFFSET: i32 = 3;
