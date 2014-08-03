use std::fmt;

// Assembler: http://alex.nisnevich.com/dcpu16-assembler/
// CPU Spec: https://raw.githubusercontent.com/gatesphere/demi-16/master/docs/dcpu-specs/dcpu-1-7.txt

#[allow(dead_code)] // REMOVE ME!
enum Op {
    Special,
    SET,
    ADD,
    SUB,
    MUL,
    MLI,
    DIV,
    DVI,
    MOD,
    MDI,
    AND,
    BOR,
    XOR,
    SHR,
    ASR,
    SHL,
    IFB,
    IFC,
    IFE,
    IFN,
    IFG,
    IFA,
    IFL,
    IFU,
    ADX = 0x1a,
    SBC = 0x1b,
    STI = 0x1e,
    STD = 0x1f
}

#[allow(dead_code)] // REMOVE ME!
enum SpecialOp {
    JSR = 0x1,
    INT = 0x8,
    IAG = 0x9,
    IAS = 0xa,
    RFI = 0xb,
    IAQ = 0xc,
    HWN = 0x10,
    HWQ = 0x11,
    HWI = 0x12
}

#[allow(dead_code)] // REMOVE ME!
struct CpuState {
    reg: [u16, .. 8],
    pc: u16,
    sp: u16,
    ex: u16,
    ia: u16,
    mem: Vec<u16>
}

impl CpuState {
    fn new() -> CpuState {
        let n = 0u16;
        CpuState {
            reg: [n, n, n, n, n, n, n, n],
            pc: n,
            sp: n,
            ex: n,
            ia: n,
            mem: Vec::from_elem(0x10000, 0u16)
        }
    }
}

impl fmt::Show for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "--------------------------\n\
                [ A: 0x{:04x}, B: 0x{:04x}, C: 0x{:04x}, X: 0x{:04x}, \
                Y: 0x{:04x}, Z: 0x{:04x}, I: 0x{:04x}, J: 0x{:04x} ]\n \
                PC: 0x{:04x}\n SP: 0x{:04x}\n EX: 0x{:04x}\n IA: 0x{:04x}\n\
                --------------------------",
               self.reg[0], self.reg[1], self.reg[2], self.reg[3],
               self.reg[4], self.reg[5], self.reg[6], self.reg[7],
               self.pc, self.sp, self.ex, self.ia)
    }
}

fn main() {
    let c = CpuState::new();
    println!("{}", c);

}
