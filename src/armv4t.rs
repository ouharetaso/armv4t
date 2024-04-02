use deku::prelude::*;

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

pub type BusState = Result<Word, ()>;

pub trait Bus {
    fn access(&mut self, addr: Word, data: &mut Word, rw: BusRW) -> BusState;
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(endian = "big")]
pub struct CpsrFlags {
    #[deku(bits=1)]
    pub n: u32,
    #[deku(bits=1)]
    pub z: u32,
    #[deku(bits=1)]
    pub c: u32,
    #[deku(bits=1)]
    pub v: u32,
    #[deku(bits=1)]
    pub q: u32,
    #[deku(bits=19)]
    pub reserved: u32,
    #[deku(bits=1)]
    pub i: u32,
    #[deku(bits=1)]
    pub f: u32,
    #[deku(bits=1)]
    pub t: u32,
    #[deku(bits=5)]
    pub mode: u32,
}

pub struct InstFormat {
    pub mask: u32,
    pub data: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct DataProcess {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=2)]
    pub _00: u32,
    #[deku(bits=1)]
    pub i: u32,
    #[deku(bits=4)]
    pub opcode: u32,
    #[deku(bits=1)]
    pub s: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=12)]
    pub operand2: u32,

}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Multiply {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=6)]
    pub _000000: u32,
    #[deku(bits=1)]
    pub a: u32,
    #[deku(bits=1)]
    pub s: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rs: u32,
    #[deku(bits=4)]
    pub _1001: u32,
    #[deku(bits=4)]
    pub rm: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct MultiplyLong {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=5)]
    pub _00001: u32,
    #[deku(bits=1)]
    pub u: u32,
    #[deku(bits=1)]
    pub a: u32,
    #[deku(bits=1)]
    pub s: u32,
    #[deku(bits=4)]
    pub rd_hi: u32,
    #[deku(bits=4)]
    pub rd_lo: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub _1001: u32,
    #[deku(bits=4)]
    pub rm: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct SingleDataSwap {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=5)]
    pub _00010: u32,
    #[deku(bits=1)]
    pub b: u32,
    #[deku(bits=2)]
    pub _00: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=8)]
    pub _00001001: u32,
    #[deku(bits=4)]
    pub rm: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct BranchExchange {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=24)]
    pub _000100101111111111110001: u32,
    #[deku(bits=4)]
    pub rn: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct HalfWordDataTransfer {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=3)]
    pub _000: u32,
    #[deku(bits=1)]
    pub p: u32,
    #[deku(bits=1)]
    pub u: u32,
    #[deku(bits=1)]
    pub i: u32,
    #[deku(bits=1)]
    pub w: u32,
    #[deku(bits=1)]
    pub l: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=4)]
    pub offset1: u32,
    #[deku(bits=1)]
    pub _1: u32,
    #[deku(bits=1)]
    pub s: u32,
    #[deku(bits=1)]
    pub h: u32,
    #[deku(bits=1)]
    pub __1: u32,
    #[deku(bits=4)]
    pub offset2: u32,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct SingleDataTransfer {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=2)]
    pub _01: u32,
    #[deku(bits=1)]
    pub i: u32,
    #[deku(bits=1)]
    pub p: u32,
    #[deku(bits=1)]
    pub u: u32,
    #[deku(bits=1)]
    pub b: u32,
    #[deku(bits=1)]
    pub w: u32,
    #[deku(bits=1)]
    pub l: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=12)]
    pub offset: u32,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct BlockDataTransfer {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=3)]
    pub _100: u32,
    #[deku(bits=1)]
    pub p: u32,
    #[deku(bits=1)]
    pub u: u32,
    #[deku(bits=1)]
    pub s: u32,
    #[deku(bits=1)]
    pub w: u32,
    #[deku(bits=1)]
    pub l: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=16)]
    pub register_list: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Branch {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=3)]
    pub _101: u32,
    #[deku(bits=1)]
    pub l: u32,
    #[deku(bits=24)]
    pub offset: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct CoProcessorDataTransfer {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=3)]
    pub _110: u32,
    #[deku(bits=1)]
    pub p: u32,
    #[deku(bits=1)]
    pub u: u32,
    #[deku(bits=1)]
    pub n: u32,
    #[deku(bits=1)]
    pub w: u32,
    #[deku(bits=1)]
    pub l: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub crd: u32,
    #[deku(bits=4)]
    pub cp_num: u32,
    #[deku(bits=8)]
    pub offset: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct CoProcessorDataOperation {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=4)]
    pub _1110: u32,
    #[deku(bits=4)]
    pub cp_opc: u32,
    #[deku(bits=4)]
    pub crn: u32,
    #[deku(bits=4)]
    pub crd: u32,
    #[deku(bits=4)]
    pub cp_num: u32,
    #[deku(bits=3)]
    pub cp: u32,
    #[deku(bits=1)]
    pub _0: u32,
    #[deku(bits=4)]
    pub crm: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct CoProcessorRegisterTransfer {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=4)]
    pub _1110: u32,
    #[deku(bits=3)]
    pub cp_opc: u32,
    #[deku(bits=1)]
    pub l: u32,
    #[deku(bits=4)]
    pub crn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=4)]
    pub cp_num: u32,
    #[deku(bits=3)]
    pub cp: u32,
    #[deku(bits=1)]
    pub _1: u32,
    #[deku(bits=4)]
    pub crm: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct SoftwareInterrupt {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=24)]
    pub _1111: u32,
    #[deku(bits=4)]
    pub imm24: u32,
}

pub struct ShifterOperand {
    pub shifter_operand: u32,
    pub carry_out: bool
}


pub enum InstKind {
    DataProcess(DataProcess),
    Multiply(Multiply),
    MultiplyLong(MultiplyLong),
    SingleDataSwap(SingleDataSwap),
    BranchExchange(BranchExchange),
    HalfWordDataTransfer(HalfWordDataTransfer),
    SingleDataTransfer(SingleDataTransfer),
    BlockDataTransfer(BlockDataTransfer),
    Branch(Branch),
    CoProcessorDataTransfer(CoProcessorDataTransfer),
    CoProcessorDataOperation(CoProcessorDataOperation),
    CoProcessorRegisterTransfer(CoProcessorRegisterTransfer),
    SoftwareInterrupt(SoftwareInterrupt),
    Undefined,
}


pub enum BusRW {
    Read,
    Write,
}


pub struct BankedRegisters {
    pub fiq: [Word; 7],
    pub irq: [Word; 2],
    pub svc: [Word; 2],
    pub abt: [Word; 2],
    pub und: [Word; 2],
}

pub struct Cpu<T: Bus> {
    pub mode: ProcessorMode,
    pub r: [Word; 16],
    pub banked: BankedRegisters,
    pub cpsr: CpsrFlags,
    pub spsr: [Word; 5],
    pub bus: T,
}


impl<T> Cpu<T>
where T: Bus
{
    pub fn step(&mut self) {
        let inst: Word = self.fetch();
        let decoded_inst = self.decode(inst);
        self.execute(decoded_inst, (inst as u32) >> 28);
        self.advance_pc(4);
    }

    pub fn execute(&mut self, decoded_inst: InstKind, cond: u32) {
        if self.is_condition_passed(cond){
            match decoded_inst {
                InstKind::DataProcess(inst) => {
                    let shifter_operand = self.get_shifter_operand(&inst);
                    let mut c = self.cpsr.c;
                    let mut v = self.cpsr.v;
                    let result: u32 = match inst.opcode {
                        // AND
                        0x0 => self.get_gpr(inst.rn as u8) & shifter_operand.shifter_operand,
                        // EOR
                        0x1 => self.get_gpr(inst.rn as u8) ^ shifter_operand.shifter_operand,
                        // SUB
                        0x2 => {
                            self.get_gpr(inst.rn as u8) - shifter_operand.shifter_operand
                        },
                        0x3 => self.get_gpr(inst.rn as u8) + shifter_operand.shifter_operand,
                        0x4 => self.get_gpr(inst.rn as u8) & shifter_operand.shifter_operand,
                        0x5 => self.get_gpr(inst.rn as u8) ^ shifter_operand.shifter_operand,
                        0x6 => self.get_gpr(inst.rn as u8) - shifter_operand.shifter_operand,
                        0x7 => self.get_gpr(inst.rn as u8) + shifter_operand.shifter_operand,
                        0x8 => self.get_gpr(inst.rn as u8) & shifter_operand.shifter_operand,
                        0x9 => self.get_gpr(inst.rn as u8) ^ shifter_operand.shifter_operand,
                        0xA => self.get_gpr(inst.rn as u8) - shifter_operand.shifter_operand,
                        0xB => self.get_gpr(inst.rn as u8) + shifter_operand.shifter_operand,
                        0xC => self.get_gpr(inst.rn as u8) & shifter_operand.shifter_operand,
                        0xD => self.get_gpr(inst.rn as u8) ^ shifter_operand.shifter_operand,
                        0xE => self.get_gpr(inst.rn as u8) - shifter_operand.shifter_operand,
                        0xF => self.get_gpr(inst.rn as u8) + shifter_operand.shifter_operand,
                        _ => 0,
                    };
                    self.set_gpr(inst.rd as u8, result);
                    if inst.s != 0 && inst.rd != 15{
                        self.cpsr.n = ((result & 0x80000000) != 0) as u32;
                        self.cpsr.z = (result == 0) as u32;
                        self.cpsr.c = c;
                        self.cpsr.v = v;
                    }
                    else {
                        self.store_spsr();
                    }
                },
                _ => (),
            }
        }
        else{
            ()
        }
    }

    pub fn is_condition_passed(&self, cond: u32) -> bool {
        let n = self.cpsr.n != 0;
        let z = self.cpsr.z != 0;
        let c = self.cpsr.c != 0;
        let v = self.cpsr.v != 0;
        match cond {
            0x0 => z,
            0x1 => !z,
            0x2 => c,
            0x3 => !c,
            0x4 => n,
            0x5 => !n,
            0x6 => v,
            0x7 => !v,
            0x8 => c && !z,
            0x9 => !c || z,
            0xA => n == v,
            0xB => n != v,
            0xC => !z && n == v,
            0xD => z || n != v,
            0xE => true,
            _ => false,
        }
    }

    pub fn get_shifter_operand(&self, inst: &DataProcess) -> ShifterOperand {
        // Immediate
        if inst.i == 0 {
            let rotate_imm: u32 = (inst.operand2 & 0xF00) >> 8;
            let imm: u32 = inst.operand2 & 0xFF;
            return ShifterOperand {
                shifter_operand: imm.rotate_right(rotate_imm * 2),
                carry_out: if rotate_imm == 0 { self.cpsr.c != 0 } else { (imm.rotate_right(rotate_imm * 2) >> 31) & 1 != 0},
            };
        }
        // Register shift
        else {
            // Register shift by immediate
            if (inst.operand2 & 0x10) == 0 {
                let shift_imm: u32 = (inst.operand2 & 0xF00) >> 8;
                let shift: u32 = (inst.operand2 & 0x60) >> 5;
                let rm: u8 = (inst.operand2 & 0xF) as u8;
                let shifter_operand: ShifterOperand = match shift {
                    // LSL
                    0 => {
                        if shift_imm == 0 {
                            let shift_carry_out = self.cpsr.c != 0;
                            ShifterOperand { shifter_operand: self.get_gpr(rm), carry_out: shift_carry_out}
                        } else {
                            let (result, _) = self.get_gpr(rm).overflowing_shl(shift_imm);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (32 - shift_imm))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                    },
                    // LSR
                    1 => {
                        if shift_imm == 0 {
                            let shift_carry_out = (self.get_gpr(rm) & 0x80000000) != 0;
                            ShifterOperand { shifter_operand: 0, carry_out: shift_carry_out}
                        } else {
                            let (result, _) = self.get_gpr(rm).overflowing_shr(shift_imm);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (shift_imm - 1))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                    },
                    // ASR
                    2 => {
                        if shift_imm == 0{
                            let shift_carry_out = (self.get_gpr(rm) & 0x80000000) != 0;
                            let result = if shift_carry_out { 0xFFFFFFFF } else { 0 };
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        } else {
                            let (result, _) = self.get_gpr(rm).overflowing_shr(shift_imm);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (shift_imm - 1))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                    },
                    // ROR
                    _ => {
                        // RRX
                        if shift_imm == 0 {
                            let shift_carry_out = (self.get_gpr(rm) & 0x00000001) != 0;
                            let result = (self.get_gpr(rm as u8) >> 1) | ((self.cpsr.c as u32) << 31);
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                        else {
                            let result = self.get_gpr(rm).rotate_right(shift_imm);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (shift_imm - 1))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                    },
                };
                return shifter_operand;
            }
            // Register shift by register
            else {
                let rs = (inst.operand2 & 0xF00) >> 8;
                let shift_amount = (self.get_gpr(rs as u8) & 0xFF);
                let shift = (inst.operand2 & 0x60) >> 5;
                let rm: u8 = (inst.operand2 & 0xF) as u8;

                let shifter_operand: ShifterOperand = match shift {
                    // LSL
                    0 => {
                        if shift_amount == 0 {
                            let shift_carry_out = self.cpsr.c != 0;
                            ShifterOperand { shifter_operand: self.get_gpr(rm), carry_out: shift_carry_out}
                        }
                        else if shift_amount < 32{
                            let (result, _) = self.get_gpr(rm).overflowing_shl(shift_amount);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (32 - shift_amount))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                        else {
                            let result = 0;
                            let shift_carry_out = if shift_amount == 32 {(self.get_gpr(rm) & 1) != 0} else {false};
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                    },
                    // LSR
                    1 => {
                        if shift_amount == 0 {
                            let shift_carry_out = (self.get_gpr(rm) & 0x80000000) != 0;
                            ShifterOperand { shifter_operand: 0, carry_out: shift_carry_out}
                        } else if shift_amount < 32 {
                            let (result, _) = self.get_gpr(rm).overflowing_shr(shift_amount);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (shift_amount - 1))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                        else {
                            let result = 0;
                            let shift_carry_out = if shift_amount == 32 {(self.get_gpr(rm) & 0x80000000) != 0} else {false};
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                    },
                    // ASR
                    2 => {
                        if shift_amount == 0 {
                            let shift_carry_out = self.cpsr.c != 0;
                            let result = self.get_gpr(rm);
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        } else if shift_amount < 32 {
                            let (result, _) = self.get_gpr(rm).overflowing_shr(shift_amount);
                            let shift_carry_out = (self.get_gpr(rm) & (1 << (shift_amount - 1))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                        else {
                            let shifter_carry_out = (self.get_gpr(rm) & 0x80000000) != 0;
                            let result = if shifter_carry_out { 0xFFFFFFFF } else { 0 };
                            ShifterOperand { shifter_operand: result, carry_out: shifter_carry_out}
                        }
                    },
                    // ROR
                    _ => {
                        if shift_amount == 0 {
                            let shift_carry_out = self.cpsr.c != 0;
                            let result = self.get_gpr(rm);
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        } else if shift_amount & 0x1f == 0 {
                            let result = self.get_gpr(rm);
                            let shift_carry_out = result & 0x80000000 != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shift_carry_out}
                        }
                        else {
                            let result = self.get_gpr(rm).rotate_right(shift_amount & 0x1f);
                            let shifter_carry_out = (self.get_gpr(rm) & (1 << ((shift_amount & 0x1f) - 1))) != 0;
                            ShifterOperand { shifter_operand: result, carry_out: shifter_carry_out}
                        }
                    }
                };
                shifter_operand
            }
        }
    }

    pub fn fetch(&mut self) -> Word {
        let pc = self.get_gpr(15);
        let mut data: Word = 0;
        self.bus.access(pc, &mut data, BusRW::Read);
        data
    }

    pub fn decode(&self, inst: Word) -> InstKind {
        const DATA_PROCESS: InstFormat                  = InstFormat{ mask: 0x0c000000, data: 0x00000000 };
        const MULTIPLY: InstFormat                      = InstFormat{ mask: 0x0FC000F0, data: 0x00000090 };
        const MULTIPLY_LONG: InstFormat                 = InstFormat{ mask: 0x0FC000F0, data: 0x00800090 };
        const SINGLE_DATA_SWAP: InstFormat              = InstFormat{ mask: 0x0FB00FF0, data: 0x01000090 };
        const BRANCH_EXCHANGE: InstFormat               = InstFormat{ mask: 0x0FFFFFF0, data: 0x012FFF10 };
        const HW_DATA_TRANSFER: InstFormat              = InstFormat{ mask: 0x0E000090, data: 0x00000090 };
        const SINGLE_DATA_TRANSFER: InstFormat          = InstFormat{ mask: 0x0C000000, data: 0x04000000 };
        const BLOCK_DATA_TRANSFER: InstFormat           = InstFormat{ mask: 0x0E000000, data: 0x08000000 };
        const BRANCH: InstFormat                        = InstFormat{ mask: 0x0E000000, data: 0x0A000000 };
        const CO_PROCESOR_DATA_TRANSFER: InstFormat     = InstFormat{ mask: 0x0E000000, data: 0x0C000000 };
        const CO_PROCESOR_DATA_OPERATION: InstFormat    = InstFormat{ mask: 0x0F000010, data: 0x0E000000 };
        const CO_PROCESOR_REGISTER_TRANSFER: InstFormat = InstFormat{ mask: 0x0F000010, data: 0x0E000010 };
        const SOFTWARE_INTERRUPT: InstFormat            = InstFormat{ mask: 0x0F000000, data: 0x0F000000 };

        if Cpu::<T>::is_match_format(inst, DATA_PROCESS) {
            let (_, data_process) = DataProcess::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::DataProcess(data_process);
        }
        else if Cpu::<T>::is_match_format(inst, MULTIPLY){
            let (_, multiply) = Multiply::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::Multiply(multiply);
        }
        else if Cpu::<T>::is_match_format(inst, MULTIPLY_LONG){
            let (_, multiply_long) = MultiplyLong::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::MultiplyLong(multiply_long);
        }
        else if Cpu::<T>::is_match_format(inst, SINGLE_DATA_SWAP){
            let (_, single_data_swap) = SingleDataSwap::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::SingleDataSwap(single_data_swap);
        }
        else if Cpu::<T>::is_match_format(inst, BRANCH_EXCHANGE){
            let (_, branch_exchange) = BranchExchange::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::BranchExchange(branch_exchange);
        }
        else if Cpu::<T>::is_match_format(inst, HW_DATA_TRANSFER){
            let (_, half_word_data_transfer) = HalfWordDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::HalfWordDataTransfer(half_word_data_transfer);
        }
        else if Cpu::<T>::is_match_format(inst, SINGLE_DATA_TRANSFER){
            let (_, single_data_transfer) = SingleDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::SingleDataTransfer(single_data_transfer);
        }
        else if Cpu::<T>::is_match_format(inst, BLOCK_DATA_TRANSFER){
            let (_, block_data_transfer) = BlockDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::BlockDataTransfer(block_data_transfer);
        }
        else if Cpu::<T>::is_match_format(inst, BRANCH){
            let (_, branch) = Branch::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::Branch(branch);
        }
        else if Cpu::<T>::is_match_format(inst, CO_PROCESOR_DATA_TRANSFER){
            let (_, co_processor_data_transfer) = CoProcessorDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::CoProcessorDataTransfer(co_processor_data_transfer);
        }
        else if Cpu::<T>::is_match_format(inst, CO_PROCESOR_DATA_OPERATION){
            let (_, co_processor_data_operation) = CoProcessorDataOperation::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::CoProcessorDataOperation(co_processor_data_operation);
        }
        else if Cpu::<T>::is_match_format(inst, CO_PROCESOR_REGISTER_TRANSFER){
            let (_, co_processor_register_transfer) = CoProcessorRegisterTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::CoProcessorRegisterTransfer(co_processor_register_transfer);
        }
        else if Cpu::<T>::is_match_format(inst, SOFTWARE_INTERRUPT){
            let (_, software_interrupt) = SoftwareInterrupt::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            return InstKind::SoftwareInterrupt(software_interrupt);
        }
        else {
            return InstKind::Undefined;
        }
    }

    pub fn is_match_format(inst: Word, format: InstFormat) -> bool {
        return (inst & format.mask) == format.data;
    }



    pub fn new(bus: T) -> Cpu<T> {
        Cpu {
            mode: ProcessorMode::Supervisor(0),
            r: [0; 16],
            banked: BankedRegisters {
                fiq: [0; 7],
                irq: [0; 2],
                svc: [0; 2],
                abt: [0; 2],
                und: [0; 2],
            },
            cpsr: CpsrFlags {
                n: 0,
                z: 0,
                c: 0,
                v: 0,
                q: 0,
                reserved: 0,
                i: 0,
                f: 0,
                t: 0,
                mode: 0,
            },
            spsr: [0; 5],
            bus: bus,
        }
    }

    pub fn advance_pc(&mut self, offset: Word) {
        self.set_gpr(15, self.get_gpr(15) + offset);
    }

    pub fn reset (&mut self) {
        self.mode = ProcessorMode::Supervisor(0);
        // Overwrite R14_svc by copying the current PC to it
        self.set_gpr(14, self.get_gpr(15));
        self.store_spsr();
        self.r = [0; 16];
        self.banked = BankedRegisters {
            fiq: [0; 7],
            irq: [0; 2],
            svc: [0; 2],
            abt: [0; 2],
            und: [0; 2],
        };
        self.spsr = [0; 5];
    }

    pub fn store_spsr(&mut self) {
        let cpsr = u32::from_be_bytes(self.get_cpsr().to_bytes().unwrap().try_into().unwrap());
        match self.mode {
            ProcessorMode::FIQ(_) => self.spsr[0] = cpsr,
            ProcessorMode::IRQ(_) => self.spsr[1] = cpsr,
            ProcessorMode::Supervisor(_) => self.spsr[2] = cpsr,
            ProcessorMode::Abort(_) => self.spsr[3] = cpsr,
            ProcessorMode::Undefined(_) => self.spsr[4] = cpsr,
            _ => (),
        }
    }

    pub fn get_gpr(&self, reg: u8) -> Word {
        match self.mode {
            ProcessorMode::User(_) => self.r[reg as usize],
            ProcessorMode::FIQ(_) => if reg < 8 { self.r[reg as usize] } else { self.banked.fiq[reg as usize - 8] },
            ProcessorMode::IRQ(_) => if reg < 13 { self.r[reg as usize] } else { self.banked.irq[reg as usize - 13] },
            ProcessorMode::Supervisor(_) => if reg < 13 { self.r[reg as usize] } else { self.banked.svc[reg as usize - 13] },
            ProcessorMode::Abort(_) => if reg < 13 { self.r[reg as usize] } else { self.banked.abt[reg as usize - 13] },
            ProcessorMode::Undefined(_) => if reg < 13 { self.r[reg as usize] } else { self.banked.und[reg as usize - 13] },
            ProcessorMode::System(_) => self.r[reg as usize],
        }
    }

    pub fn set_gpr(&mut self, reg: u8, value: Word) {
        match self.mode {
            ProcessorMode::User(_) => self.r[reg as usize] = value,
            ProcessorMode::FIQ(_) => if reg < 8 { self.r[reg as usize] = value } else { self.banked.fiq[reg as usize - 8] = value },
            ProcessorMode::IRQ(_) => if reg < 13 { self.r[reg as usize] = value } else { self.banked.irq[reg as usize - 13] = value },
            ProcessorMode::Supervisor(_) => if reg < 13 { self.r[reg as usize] = value } else { self.banked.svc[reg as usize - 13] = value },
            ProcessorMode::Abort(_) => if reg < 13 { self.r[reg as usize] = value } else { self.banked.abt[reg as usize - 13] = value },
            ProcessorMode::Undefined(_) => if reg < 13 { self.r[reg as usize] = value } else { self.banked.und[reg as usize - 13] = value },
            ProcessorMode::System(_) => self.r[reg as usize] = value,
        }
    }


    pub fn get_mode(&self) -> &ProcessorMode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: ProcessorMode) {
        self.mode = mode;
    }

    pub fn get_cpsr(&self) -> CpsrFlags {
        self.cpsr
    }

    pub fn set_cpsr(&mut self, value: CpsrFlags) {
        self.cpsr = value;
    }

    pub fn get_spsr(&self) -> Word {
        match self.mode {
            ProcessorMode::FIQ(_) => self.spsr[0],
            ProcessorMode::IRQ(_) => self.spsr[1],
            ProcessorMode::Supervisor(_) => self.spsr[2],
            ProcessorMode::Abort(_) => self.spsr[3],
            ProcessorMode::Undefined(_) => self.spsr[4],
            _ => 0,
        }
    }

    pub fn set_spsr(&mut self, value: Word) {
        match self.mode {
            ProcessorMode::FIQ(_) => self.spsr[0] = value,
            ProcessorMode::IRQ(_) => self.spsr[1] = value,
            ProcessorMode::Supervisor(_) => self.spsr[2] = value,
            ProcessorMode::Abort(_) => self.spsr[3] = value,
            ProcessorMode::Undefined(_) => self.spsr[4] = value,
            _ => (),
        }
    }
}