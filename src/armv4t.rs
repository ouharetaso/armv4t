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


#[derive(Clone)]
pub enum Mnemonic{
    ADC,        // impl
    ADD,        // impl
    AND,        // impl
    B,          // impl
    BL,         // impl
    BIC,        // impl
    BX,
    CMN,        // impl
    CMP,        // impl
    EOR,        // impl
    LDC,        
    LDM,
    LDR,
    LDRB,
    LDRBT,
    LDRH,
    LDRT,
    MCR,
    MLA,
    MOV,        // impl
    MRC,
    MRS,
    MSR,
    MUL,
    MVN,        // impl
    ORR,        // impl
    RSB,        // impl
    RSC,        // impl
    SBC,        // impl
    SMLAL,
    SMULL,
    STC,
    STM,
    STR,
    STRB,
    STRBT,
    STRH,
    STRT,
    SUB,        // impl
    SWI,
    SWP,
    SWPB,
    TEQ,        // impl
    TST,        // impl
    UMLAL,
    UMULL,
    UND,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(endian = "big")]
pub struct ControlImmediate {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=5)]
    pub _00010: u32,
    #[deku(bits=2)]
    pub op1: u32,
    #[deku(bits=1)]
    pub _0: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=4)]
    pub rotate_imm: u32,
    #[deku(bits=8)]
    pub immed_8: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(endian = "big")]
pub struct ControlRegister {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=5)]
    pub _00010: u32,
    #[deku(bits=2)]
    pub op1: u32,
    #[deku(bits=1)]
    pub _0: u32,
    #[deku(bits=4)]
    pub rn: u32,
    #[deku(bits=4)]
    pub rd: u32,
    #[deku(bits=4)]
    pub rs: u32,
    #[deku(bits=4)]
    pub op2: u32,
    #[deku(bits=4)]
    pub rm: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(endian = "big")]
pub struct BranchExchange {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=24)]
    pub _000100101111111111110001: u32,
    #[deku(bits=4)]
    pub rn: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(endian = "big")]
pub struct LoadStoreExtention {
    #[deku(bits=4)]
    pub cond: u32,
    #[deku(bits=3)]
    pub _000: u32,
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
    #[deku(bits=4)]
    pub rs: u32,
    #[deku(bits=1)]
    pub _1: u32,
    #[deku(bits=2)]
    pub op1: u32,
    #[deku(bits=1)]
    pub __1: u32,
    #[deku(bits=4)]
    pub offset2: u32,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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


#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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


#[derive(Copy, Clone)]
pub enum InstKind {
    DataProcess(DataProcess),
    Multiply(Multiply),
    ControlImmediate(ControlImmediate),
    ControlRegister(ControlRegister),
    BranchExchange(BranchExchange),
    LoadStoreExtention(LoadStoreExtention),
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


pub enum PipelineState {
    Fetch,
    Decode(Word),
    Execute(InstKind),
}


pub struct DecodedInstruction {
    pub inst: InstKind,
    pub cond: u32,
    pub raw_inst: u32,
}

pub struct ARMv4T<T: Bus> {
    pub mode: ProcessorMode,
    pub r: [Word; 16],
    pub banked: BankedRegisters,
    pub cpsr: CpsrFlags,
    pub spsr: [Word; 5],
    pub bus: T,
    pub inst: Option<Word>,
    pub decoded_inst : Option<DecodedInstruction>,
}


impl<T> ARMv4T<T>
where T: Bus
{
    pub fn step(&mut self) {
        let mut decoded_inst: Option<DecodedInstruction> = None;
        match self.inst {
            Some(inst) => {
                decoded_inst = Some(self.decode(inst));
            }
            None => (),
        }
        self.inst = Some(self.fetch());

        match &self.decoded_inst {
            Some(decoded) => {
                let is_pc_changed = self.execute(decoded.inst, decoded.cond);
                if is_pc_changed {
                    self.flush_pipeline();
                }
                else {
                    self.advance_pc(0x4);
                    self.decoded_inst = decoded_inst;
                }
            }
            None => {
                self.advance_pc(0x4);
                self.decoded_inst = decoded_inst;
            },
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.inst = None;
        self.decoded_inst = None;
    }

    pub fn execute(&mut self, decoded_inst: InstKind, cond: u32) -> bool {
        let mut is_pc_changed = false;
        if self.is_condition_passed(cond){
            match decoded_inst {
                InstKind::BlockDataTransfer(inst) => {
                    let start_address = if inst.u == 1 && inst.p == 0 {
                        // increment after
                        self.get_gpr(inst.rn as u8)
                    }
                    else if inst.u == 1 && inst.p == 1 {
                        // increment before
                        self.get_gpr(inst.rn as u8) + 4
                    }
                    else if inst.u == 0 && inst.p == 0 {
                        // decrement after
                        self.get_gpr(inst.rn as u8) - inst.register_list.count_ones() as u32 * 4 + 4
                    }
                    else {
                        // decrement before
                        self.get_gpr(inst.rn as u8) - inst.register_list.count_ones() as u32 * 4
                    };
                    
                    let mut address = start_address;

                    for i in 0..16 {
                        if inst.register_list & (1 << i) != 0 {
                            if inst.l == 1 {
                                let mut data: Word = 0;
                                _ = self.bus.access(address, &mut data, BusRW::Read);
                                self.set_gpr(i as u8, data);
                            }
                            else {
                                let mut data = self.get_gpr(i as u8);
                                _ = self.bus.access(address, &mut data, BusRW::Write);
                            }
                            address = if inst.u == 1 {address + 4} else {address - 4};
                        }
                    
                    }
                }
                InstKind::SingleDataTransfer(inst) => {
                    let rn = self.get_gpr(inst.rn as u8);
                    let offset: u32;
                    if inst.i != 0 {
                        let rm  = self.get_gpr(get_bit_range(inst.offset, 3, 0) as u8);
                        let shift_imm = get_bit_range(inst.offset, 11, 7);
                        offset = match get_bit_range(inst.offset, 6, 5) {
                            0b00 => rm << shift_imm,
                            0b01 => rm >> shift_imm,
                            0b10 => if rm & 0x80000000 != 0 {
                                (rm >> shift_imm) | (0xFFFFFFFF << (32 - shift_imm))
                            }
                            else {
                                rm >> shift_imm
                            }
                            0b11 => if shift_imm != 0 {rm.rotate_right(shift_imm)} else {(rm >> 1)| ((self.cpsr.c as u32) << 31)},
                            _ => 0,
                        };
                    }
                    else {
                        offset = inst.offset;
                    }
                    let address = if inst.u != 0 { rn + offset } else { rn - offset };

                    // normal operation
                    if inst.p == 1 && inst.w == 0 {
                        if inst.l != 0 {
                            let mut data: Word = 0;
                            _ =  self.bus.access(address, &mut data, BusRW::Read);
                            self.set_gpr(inst.rd as u8, data);
                        }
                        else {
                            let mut data = self.get_gpr(inst.rd as u8);
                            _ = self.bus.access(address, &mut data, BusRW::Write);
                        }
                        ()
                    }
                    // pre-indexed
                    if inst.p == 1 && inst.w == 1 {
                        if inst.l != 0 {
                            let mut data: Word = 0;
                            _ =  self.bus.access(address, &mut data, BusRW::Read);
                            self.set_gpr(inst.rd as u8, data);
                        }
                        else {
                            let mut data = self.get_gpr(inst.rd as u8);
                            _ = self.bus.access(address, &mut data, BusRW::Write);
                        }
                        self.set_gpr(inst.rn as u8, address);
                        ()
                    }
                    // post-indexed
                    if inst.p == 0 && inst.w == 0 {
                        if inst.l != 0 {
                            let mut data: Word = 0;
                            _ =  self.bus.access(rn, &mut data, BusRW::Read);
                            self.set_gpr(inst.rd as u8, data);
                        }
                        else {
                            let mut data = self.get_gpr(inst.rd as u8);
                            _ = self.bus.access(rn, &mut data, BusRW::Write);
                        }
                        self.set_gpr(inst.rn as u8, address);
                        ()
                    }
                }
                InstKind::DataProcess(inst) => {
                    let shifter_operand = self.get_shifter_operand(&inst);
                    let mut n = self.cpsr.n;
                    let mut z = self.cpsr.z;
                    let mut c = self.cpsr.c;
                    let mut v = self.cpsr.v;

                    if inst.rd == 15 {
                        is_pc_changed = true;
                    }
                    let result: u32 = match inst.opcode {
                        // AND
                        0x0 => {
                            let _result = self.get_gpr(inst.rn as u8) & shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            _result
                        },
                        // EOR
                        0x1 => {
                            let _result = self.get_gpr(inst.rn as u8) ^ shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            _result
                        },
                        // SUB
                        0x2 => {
                            let (_result, _v) = self.get_gpr(inst.rn as u8).overflowing_sub(shifter_operand.shifter_operand);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = _v as u32;
                            c = (self.get_gpr(inst.rn as u8) >= shifter_operand.shifter_operand) as u32;
                            _result
                        },
                        // RSB
                        0x3 => {
                            let (_result, _v) = shifter_operand.shifter_operand.overflowing_sub(self.get_gpr(inst.rn as u8));
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = _v as u32;
                            c = !(self.get_gpr(inst.rn as u8) >= shifter_operand.shifter_operand) as u32;
                            _result
                        },
                        // ADD
                        0x4 => {
                            let (_result, _v) = self.get_gpr(inst.rn as u8).overflowing_add(shifter_operand.shifter_operand);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = _v as u32;
                            c = ((self.get_gpr(inst.rn as u8) as u64) + (shifter_operand.shifter_operand as u64) >= 0x100000000) as u32;
                            _result
                        },
                        // ADC
                        0x5 => {
                            let (_result, _v) = self.get_gpr(inst.rn as u8).overflowing_add(shifter_operand.shifter_operand + self.cpsr.c);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = _v as u32;
                            c = ((self.get_gpr(inst.rn as u8) as u64) + (shifter_operand.shifter_operand as u64) + (self.cpsr.c as u64) >= 0x100000000) as u32;
                            _result
                        },
                        // SBC
                        0x6 => {
                            let (_result, _v) = self.get_gpr(inst.rn as u8).overflowing_sub(shifter_operand.shifter_operand + self.cpsr.c);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = _v as u32;
                            c = !(self.get_gpr(inst.rn as u8) >= shifter_operand.shifter_operand + self.cpsr.c) as u32;
                            _result
                        },
                        // RSC
                        0x7 => {
                            let (_result, _v) = shifter_operand.shifter_operand.overflowing_sub(self.get_gpr(inst.rn as u8) + self.cpsr.c);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = _v as u32;
                            c = !(self.get_gpr(inst.rn as u8) >= shifter_operand.shifter_operand + self.cpsr.c) as u32;
                            _result
                        },
                        // TST
                        0x8 => {
                            let _result = self.get_gpr(inst.rn as u8) & shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            self.get_gpr(inst.rd as u8)
                        },
                        // TEQ
                        0x9 => {
                            let _result = self.get_gpr(inst.rn as u8) ^ shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            self.get_gpr(inst.rd as u8)
                        },
                        // CMP
                        0xA => {
                            let (_result, _c) = self.get_gpr(inst.rn as u8).overflowing_sub(shifter_operand.shifter_operand);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = check_sub_overflow(self.get_gpr(inst.rn as u8), shifter_operand.shifter_operand, _result) as u32;
                            c = _c as u32;
                            self.get_gpr(inst.rd as u8)
                        },
                        // CMN
                        0xB => {
                            let (_result, _c) = shifter_operand.shifter_operand.overflowing_add(self.get_gpr(inst.rn as u8));
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            v = check_add_overflow(self.get_gpr(inst.rn as u8), shifter_operand.shifter_operand, _result) as u32;
                            c = _c as u32;
                            self.get_gpr(inst.rd as u8)
                        },
                        // ORR
                        0xC => {
                            let _result = self.get_gpr(inst.rn as u8) | shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            _result
                        },
                        // MOV
                        0xD => {
                            let _result = shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            shifter_operand.shifter_operand
                        },
                        // BIC
                        0xE => {
                            let _result = self.get_gpr(inst.rn as u8) & (!shifter_operand.shifter_operand);
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            _result
                        },
                        // MVN
                        0xF => {
                            let _result = !shifter_operand.shifter_operand;
                            n = ((_result & 0x80000000) != 0) as u32;
                            z = (_result == 0) as u32;
                            c = shifter_operand.carry_out as u32;
                            shifter_operand.shifter_operand
                        }
                        _ => 0,
                    };
                    self.set_gpr(inst.rd as u8, result);
                    if inst.s != 0 && inst.rd != 15{
                        self.cpsr.n = n;
                        self.cpsr.z = z;
                        self.cpsr.c = c;
                        self.cpsr.v = v;
                    }
                    else if inst.rd == 15 {
                        self.store_spsr();
                    }
                    else {
                        ()
                    }
                },
                InstKind::Branch(inst) => {
                    let offset = inst.offset << 2;
                    let link = inst.l != 0;
                    if link {
                        self.set_gpr(14, self.get_gpr(15) - 4);
                    }
                    self.set_gpr(   15, self.get_gpr(15) + offset);
                    is_pc_changed = true;
                },
                _ => {
                    println!("{}", self);
                    panic!("Undefined instruction");
                },
            }
            is_pc_changed
        }
        else{
            is_pc_changed
        }
    }

    pub fn decode(&self, inst: Word) -> DecodedInstruction {
        const DATA_PROCESS: InstFormat                  = InstFormat{ mask: 0x0c000000, data: 0x00000000 };
        const MULTIPLY: InstFormat                      = InstFormat{ mask: 0x0FC000F0, data: 0x00000090 };
        const CONTROL_IMM: InstFormat                   = InstFormat{ mask: 0x0F900000, data: 0x01000000 };
        const CONTROL_REG1: InstFormat                  = InstFormat{ mask: 0x0F900010, data: 0x03000000 };
        const CONTROL_REG2: InstFormat                  = InstFormat{ mask: 0x0F900090, data: 0x00000010 };
        const LOAD_STORE_EXTENTION: InstFormat          = InstFormat{ mask: 0x0E000090, data: 0x00000090 };
        const BRANCH_EXCHANGE: InstFormat               = InstFormat{ mask: 0x0FFFFFF0, data: 0x012FFF10 };
        const SINGLE_DATA_TRANSFER: InstFormat          = InstFormat{ mask: 0x0C000000, data: 0x04000000 };
        const BLOCK_DATA_TRANSFER: InstFormat           = InstFormat{ mask: 0x0E000000, data: 0x08000000 };
        const BRANCH: InstFormat                        = InstFormat{ mask: 0x0E000000, data: 0x0A000000 };
        const CO_PROCESOR_DATA_TRANSFER: InstFormat     = InstFormat{ mask: 0x0E000000, data: 0x0C000000 };
        const CO_PROCESOR_DATA_OPERATION: InstFormat    = InstFormat{ mask: 0x0F000010, data: 0x0E000000 };
        const CO_PROCESOR_REGISTER_TRANSFER: InstFormat = InstFormat{ mask: 0x0F000010, data: 0x0E000010 };
        const SOFTWARE_INTERRUPT: InstFormat            = InstFormat{ mask: 0x0F000000, data: 0x0F000000 };

        let cond: u32 = (inst & 0xF0000000) >> 28;
        let mut inst_kind: InstKind = InstKind::Undefined;

        if is_match_format(inst, DATA_PROCESS) {
            // arithmetic extention
            if is_match_format(inst, MULTIPLY){
                let (_, multiply) = Multiply::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
                inst_kind = InstKind::Multiply(multiply);
            }
            // control extention
            else if is_match_format(inst, CONTROL_IMM) {
                let (_, control_extentsion) = ControlImmediate::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
                inst_kind = InstKind::ControlImmediate(control_extentsion);
            }
            // load/store extension
            else if is_match_format(inst, CONTROL_REG1) || is_match_format(inst, CONTROL_REG2) {
                let (_, control_register) = ControlRegister::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
                inst_kind = InstKind::ControlRegister(control_register);
            }
            else {
                let (_, data_process) = DataProcess::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
                inst_kind = InstKind::DataProcess(data_process);
            }
        }
        else if is_match_format(inst, BRANCH_EXCHANGE){
            let (_, branch_exchange) = BranchExchange::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::BranchExchange(branch_exchange);
        }
        else if is_match_format(inst, SINGLE_DATA_TRANSFER){
            let (_, single_data_transfer) = SingleDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::SingleDataTransfer(single_data_transfer);
        }
        else if is_match_format(inst, BLOCK_DATA_TRANSFER){
            let (_, block_data_transfer) = BlockDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::BlockDataTransfer(block_data_transfer);
        }
        else if is_match_format(inst, BRANCH){
            let (_, branch) = Branch::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::Branch(branch);
        }
        else if is_match_format(inst, CO_PROCESOR_DATA_TRANSFER){
            let (_, co_processor_data_transfer) = CoProcessorDataTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::CoProcessorDataTransfer(co_processor_data_transfer);
        }
        else if is_match_format(inst, CO_PROCESOR_DATA_OPERATION){
            let (_, co_processor_data_operation) = CoProcessorDataOperation::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::CoProcessorDataOperation(co_processor_data_operation);
        }
        else if is_match_format(inst, CO_PROCESOR_REGISTER_TRANSFER){
            let (_, co_processor_register_transfer) = CoProcessorRegisterTransfer::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::CoProcessorRegisterTransfer(co_processor_register_transfer);
        }
        else if is_match_format(inst, SOFTWARE_INTERRUPT){
            let (_, software_interrupt) = SoftwareInterrupt::from_bytes((inst.to_be_bytes().as_ref(), 0)).unwrap();
            inst_kind = InstKind::SoftwareInterrupt(software_interrupt);
        }
        else {
            inst_kind = InstKind::Undefined;
        }
        DecodedInstruction {
            inst: inst_kind,
            cond: cond,
            raw_inst: inst 
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
        if inst.i != 0 {
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
                let shift_amount = self.get_gpr(rs as u8) & 0xFF;
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
        _ = self.bus.access(pc, &mut data, BusRW::Read);
        data
    }


    pub fn new(bus: T) -> ARMv4T<T> {
        ARMv4T {
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
            inst: None,
            decoded_inst: None,
        }
    }

    pub fn advance_pc(&mut self, offset: Word) {
        self.r[15] += offset;
    }

    pub fn reset (&mut self) {
        self.mode = ProcessorMode::Supervisor(0);
        // Overwrite R14_svc by copying the current PC to it
        self.set_gpr(14, self.get_gpr(15));
        self.set_gpr(15, 0x00000000);
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
            ProcessorMode::FIQ(_) => if reg < 8 || reg == 15 { self.r[reg as usize] } else { self.banked.fiq[reg as usize - 8] },
            ProcessorMode::IRQ(_) => if reg < 13 || reg == 15 { self.r[reg as usize] } else { self.banked.irq[reg as usize - 13] },
            ProcessorMode::Supervisor(_) => if reg < 13 || reg == 15 { self.r[reg as usize] } else { self.banked.svc[reg as usize - 13] },
            ProcessorMode::Abort(_) => if reg < 13 || reg == 15 { self.r[reg as usize] } else { self.banked.abt[reg as usize - 13] },
            ProcessorMode::Undefined(_) => if reg < 13 || reg == 15 { self.r[reg as usize] } else { self.banked.und[reg as usize - 13] },
            ProcessorMode::System(_) => self.r[reg as usize],
        }
    }

    pub fn set_gpr(&mut self, reg: u8, value: Word) {
        match self.mode {
            ProcessorMode::User(_) => self.r[reg as usize] = value,
            ProcessorMode::FIQ(_) => if reg < 8 || reg == 15 { self.r[reg as usize] = value } else { self.banked.fiq[reg as usize - 8] = value },
            ProcessorMode::IRQ(_) => if reg < 13 || reg == 15 { self.r[reg as usize] = value } else { self.banked.irq[reg as usize - 13] = value },
            ProcessorMode::Supervisor(_) => if reg < 13 || reg == 15 { self.r[reg as usize] = value } else { self.banked.svc[reg as usize - 13] = value },
            ProcessorMode::Abort(_) => if reg < 13 || reg == 15 { self.r[reg as usize] = value } else { self.banked.abt[reg as usize - 13] = value },
            ProcessorMode::Undefined(_) => if reg < 13 || reg == 15 { self.r[reg as usize] = value } else { self.banked.und[reg as usize - 13] = value },
            ProcessorMode::System(_) => self.r[reg as usize] = value,
        }
    }


    pub fn get_mode(&self) -> &ProcessorMode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: ProcessorMode) {
        self.mode = mode;
        match self.mode {
            ProcessorMode::User(_) => self.cpsr.mode = 0x10,
            ProcessorMode::FIQ(_) => self.cpsr.mode = 0x11,
            ProcessorMode::IRQ(_) => self.cpsr.mode = 0x12,
            ProcessorMode::Supervisor(_) => self.cpsr.mode = 0x13,
            ProcessorMode::Abort(_) => self.cpsr.mode = 0x17,
            ProcessorMode::Undefined(_) => self.cpsr.mode = 0x1B,
            ProcessorMode::System(_) => self.cpsr.mode = 0x1F,
        }   
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


pub fn is_match_format(inst: Word, format: InstFormat) -> bool {
    return (inst & format.mask) == format.data;
}


pub fn get_bit_range(data: Word, msb: u8, lsb: u8) -> Word {
    if lsb > msb {
        return 0;
    }

    let mask: u32 = ((1 << (msb - lsb + 1)) - 1) << lsb;
    return (data & mask) >> lsb;
}


pub fn disassemble(inst: Word) -> String {
    let mut mnemonic: String = "".to_string();
    let mut operand: String = "".to_string();
    let cond = get_bit_range(inst, 31, 28);

    let parse_shifter_operand =  |shifter_operand: u32, i: bool| -> String {
        if i {
            let rotate_imm = get_bit_range(shifter_operand, 11, 8);
            let immed_8 = get_bit_range(shifter_operand, 7, 0);
            format!("#{}", immed_8.rotate_right(rotate_imm * 2))
        }
        else {
            let shift = match get_bit_range(shifter_operand, 6, 5) {
                0b00 => if get_bit_range(shifter_operand, 11, 4) == 0b00000000 {
                    format!("r{}", get_bit_range(shifter_operand, 3, 0))
                }
                else {
                    "LSL".to_string()
                }
                0b01 => "LSR".to_string(),
                0b10 => "ASR".to_string(),
                0b11 => if get_bit_range(shifter_operand, 11, 4) == 0b00000110 {
                    "RRX".to_string()
                }
                else {
                    "ROR".to_string()
                },
                _ => "".to_string(),              
            };

            
            let shift_amount = if get_bit_range(shifter_operand, 11, 4) == 0b00000110 {
                // RRX
                "".to_string()
            }
            else if get_bit_range(shifter_operand, 11, 4) == 0b00000000 {
                // LSL #0
                "".to_string()
            }
            // immediate shift
            else if get_bit_range(shifter_operand, 4, 4) == 0 {
                format!("#{}", get_bit_range(shifter_operand, 11, 7))
            }
            // regsiter shift
            else {
                format!("r{}", get_bit_range(shifter_operand, 11, 8))
            };

            format!("{} {}", shift, shift_amount)
        }
    };

    let push_cond_mnemonic = |mnemonic: &mut String| -> () {
        mnemonic.push_str(match cond {
            0b0000 => "eq",
            0b0001 => "ne",
            0b0010 => "cs",
            0b0011 => "cc",
            0b0100 => "mi",
            0b0101 => "pl",
            0b0110 => "vs",
            0b0111 => "vc",
            0b1000 => "hi",
            0b1001 => "ls",
            0b1010 => "ge",
            0b1011 => "lt",
            0b1100 => "gt",
            0b1101 => "le",
            0b1110 => "",
            0b1111 => "nv",
            _ => "",
        });
    };

    // ADC
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00A00000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"adcs"} else {"adc"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // ADD
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00800000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"adds"} else {"add"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // AND
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00800000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"ands"} else {"and"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // BIC
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x01C00000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"bics"} else {"bic"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // CMN
    if is_match_format(inst, InstFormat{mask: 0x0DF00000, data: 0x01700000}){
        mnemonic.push_str("cmn");
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // CMP
    if is_match_format(inst, InstFormat{mask: 0x0DF00000, data: 0x01500000}){
        mnemonic.push_str("cmp");
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // TST
    if is_match_format(inst, InstFormat{mask: 0x0DF00000, data: 0x01100000}){
        mnemonic.push_str("tst");
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // TEQ
    if is_match_format(inst, InstFormat{mask: 0x0DF00000, data: 0x01300000}){
        mnemonic.push_str("teq");
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // SBC
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00C00000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"sbcs"} else {"sbc"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // SUB
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00400000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"subs"} else {"sub"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // RSC
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00E00000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"rscs"} else {"rsc"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // RSB
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00600000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"rsbs"} else {"rsb"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // MOV
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x01A00000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"movs"} else {"mov"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // MVN
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x01E00000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"mvns"} else {"mvn"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // EOR
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x00200000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"eors"} else {"eor"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }
    // ORR
    if is_match_format(inst, InstFormat{mask: 0x0DE00000, data: 0x01800000}){
        mnemonic.push_str(if get_bit_range(inst, 20, 20) == 1 {"orrs"} else {"orr"});
        push_cond_mnemonic(&mut mnemonic);
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 15, 12)));
        operand.push_str(&format!("r{}, ", get_bit_range(inst, 19, 16)));
        operand.push_str(&parse_shifter_operand(get_bit_range(inst, 11, 0), get_bit_range(inst, 25, 25) != 0));
    }

    format!("{:<6}  {}", mnemonic, operand)
}




pub fn check_add_overflow(a: u32, b: u32, result: u32) -> bool {
    let a_sign = (a & 0x80000000) != 0;
    let b_sign = (b & 0x80000000) != 0;
    let result_sign = (result & 0x80000000) != 0;
    (a_sign == b_sign) && (a_sign != result_sign)
}

pub fn check_sub_overflow(a: u32, b: u32, result: u32) -> bool {
    let a_sign = (a & 0x80000000) != 0;
    let b_sign = (b & 0x80000000) != 0;
    let result_sign = (result & 0x80000000) != 0;
    (a_sign != b_sign) && (a_sign != result_sign)
}

pub fn check_carry(a: u32, b: u32, result: u32) -> bool {
    let a_sign = (a & 0x80000000) != 0;
    let b_sign = (b & 0x80000000) != 0;
    let result_sign = (result & 0x80000000) != 0;
    (a_sign && b_sign) || (a_sign && !result_sign) || (b_sign && !result_sign)
}



impl<T: Bus> std::fmt::Display for ARMv4T<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut formatted_string: String = "".to_string();


        for i in 0..16 {
            formatted_string.push_str(&format!("r{:<2}: 0x{:08x}", i, self.get_gpr(i as u8)));
            if i % 4 == 3 {
                formatted_string.push_str("\n");
            }
            else {
                formatted_string.push_str(" ");
            }
        }
        
        if let Some(decoded_inst) = &self.decoded_inst {
            formatted_string.push_str(&format!("\n{}\n", disassemble(decoded_inst.raw_inst)));
        }

        write!(f, "{}", formatted_string)
    }
}