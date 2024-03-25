

type Byte = u8;
type HalfWord = u16;
type Word = u32;

#[repr(u8)]
pub enum ProcessorMode {
    User(u8) = 0x10,
    FIQ(u8) = 0x11,
    IRQ(u8) = 0x12,
    Supervisor(u8) = 0x13,
    Abort(u8) = 0x17,
    Undefined(u8) = 0x1B,
    System(u8) = 0x1F,
}

pub struct Cpu {
    pub mode: ProcessorMode,
    pub r: [Word; 16],
    pub cpsr: Word,
    pub spsr: [Word; 5],
}