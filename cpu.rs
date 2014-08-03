use std::fmt;
use std::num::FromPrimitive;

// Assembler: http://alex.nisnevich.com/dcpu16-assembler/
// CPU Spec: https://raw.githubusercontent.com/gatesphere/demi-16/master/docs/dcpu-specs/dcpu-1-7.txt

#[allow(dead_code)] // REMOVE ME!
#[deriving(PartialEq, FromPrimitive, Show)]
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
    NOOP,
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
enum Value {
    RegVal,            // Value in register x
    RegPointer,        // Value in mem[register x]
    SPushPop,          // Push if b, Pop if a
    SPeek,             // [SP]
    SPick,             // [SP + next]
    SP,
    PC,
    EX,
    NextPointer,       // mem[next word]
    NextVal,           // next word (literal)
    Val                // literal value (0xffff-0x1e) (only for a)
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
            mem: Vec::from_elem(0x10000, 0u16) // Look for immutable vec in stdlib..
        }
    }

    fn get_next_instruction(&self) -> Instruction {
        parse_instruction(*self.mem.get(self.pc as uint))
    }

    fn step(&self) -> CpuState {
        let instr = self.get_next_instruction();

        println!("Step: {}", instr);

        CpuState {
            reg: self.reg,
            pc: self.pc + 1,
            sp: self.sp,
            ex: self.ex,
            ia: self.ia,
            mem: self.mem.clone()
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

#[allow(dead_code)] // REMOVE ME!
struct Instruction {
    op: Op,
    a: u16,
    b: u16
}

impl fmt::Show for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:04x}, {:04x})", self.op, self.a, self.b)
    }
}

/*
Instructions are 1-3 words long and are fully defined by the first word.
In a basic instruction, the lower five bits of the first word of the instruction
are the opcode, and the remaining eleven bits are split into a five bit value b
and a six bit value a.
b is always handled by the processor after a, and is the lower five bits.
In bits (in LSB-0 format), a basic instruction has the format: aaaaaabbbbbooooo
*/
fn parse_instruction(instr: u16) -> Instruction {
    let hop = instr & 0b11111;
    let b = (instr >> 5) & 0b11111;
    let a = instr >> 10;
    let op: Option<Op> = FromPrimitive::from_u16(hop);

    match op {
        Some(op) => Instruction { op: op, a: a, b: b },
        None => fail!("Invalid instruction")
    }
}

fn main() {
    let c = CpuState::new();
    println!("{}", c);
    let i = parse_instruction(0x7c01u16);
    println!("{}", i);

    let j = parse_instruction(0x7803u16);
    println!("{}", j);

    c.step();
}
