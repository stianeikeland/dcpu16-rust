use std::fmt;
use std::num::FromPrimitive;
use std::io::File;

// Assembler: http://alex.nisnevich.com/dcpu16-assembler/
// CPU Spec: https://raw.githubusercontent.com/gatesphere/demi-16/master/docs/dcpu-specs/dcpu-1-7.txt

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

#[deriving(PartialEq, FromPrimitive, Show)]
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

#[deriving(PartialEq, FromPrimitive, Show)]
enum ValueType {
    Reg,            // Value in register x
    RegPointer,        // Value in mem[register x]
    RegNextPointer,    // mem[register + next word].. ?
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

impl ValueType {
    fn new(v: u16) -> ValueType {
        match v {
            0x0 .. 0x7 => Reg,
            0x8 .. 0xf => RegPointer,
            0x10 .. 0x17 => RegNextPointer,
            0x18 => SPushPop,
            0x19 => SPeek,
            0x1a => SPick,
            0x1b => SP,
            0x1c => PC,
            0x1d => EX,
            0x1e => NextPointer,
            0x1f => NextVal,
            0x20 .. 0x3f => Val,
            _ => fail!("source not implemented")
        }
    }
}

#[allow(dead_code)] // REMOVE ME!
pub struct CpuState {
    pub reg: [u16, .. 8],
    pub pc: u16,
    pub sp: u16,
    pub ex: u16,
    pub ia: u16,
    pub mem: Vec<u16>
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState {
            reg: [0,0,0,0,0,0,0,0],
            pc: 0,
            sp: 0,
            ex: 0,
            ia: 0,
            mem: Vec::from_elem(0x10000, 0u16) // Look for immutable vec in stdlib..
        }
    }

    fn instruction_fetch(&self) -> Instruction {
        Instruction::new(*self.mem.get(self.pc as uint))
    }

    // FIXME Move me..
    fn get_value_a(&self, i: Instruction) -> u16 {
        match i.a {
            Reg => self.reg[i.a_raw as uint],
            NextVal => *self.mem.get(self.pc as uint + 1),
            _ => fail!("source not implemented")
        }
    }

    // FIXME: How to handle next word for b? is it needed?
    fn get_value_b(&self, i: Instruction) -> u16 {
        match i.b {
            Reg => self.reg[i.b_raw as uint],
            _ => fail!("source not implemented")
        }
    }

    fn set_value(self, i: Instruction, val: u16) -> CpuState {
        match i.b {
            Reg => {
                let mut newreg = self.reg;
                newreg[i.b_raw as uint] = val;
                CpuState { reg: newreg, .. self }
            }
            _ => fail!("target not implemented")
        }
    }

    pub fn set_program(self, p: &Vec<u16>) -> CpuState {
        let mut m = p.clone();
        m.grow(0x10000 - p.len(), &(0u16));
        CpuState { mem: m,  .. self }
    }

    pub fn step(self) -> CpuState {
        let instr = self.instruction_fetch();
        println!("Executing: {}", instr);

        let val = self.get_value_a(instr);
        let old = self.get_value_b(instr);
        println!("val: {:04x}, old: {:04x}", val, old);

        let cpu = match instr.op {
            SET => self.set_value(instr, val),
            ADD => {
                // FIXME: Handle overflow
                self.set_value(instr, old + val)
            },
            SUB => {
                // FIXME: Handle underflow
                self.set_value(instr, old - val)
            },
            MUL => {
                // FIXME: Handle overflow
                self.set_value(instr, old * val)
            },
            DIV => {
                // FIXME: handle div by 0
                self.set_value(instr, old / val)
            },
            MOD => {
                let res = if val != 0 { old % val } else { 0 };
                self.set_value(instr, res)
            },
            AND => self.set_value(instr, old & val),
            BOR => self.set_value(instr, old | val),
            XOR => self.set_value(instr, old ^ val),
            SHR => self.set_value(instr, old >> val as uint), // FIXME: set EX
            ASR => self.set_value(instr, old << val as uint), // FIXME: set EX
            _ => fail!("op not implemented")
        };

        // Increment PC, twice if a is a [pc++] next word:
        let pc = match instr.a {
            NextVal => cpu.pc + 2,
            _ => cpu.pc + 1
        };

        CpuState { pc: pc, .. cpu }
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
    a_raw: u16,
    a: ValueType,
    b_raw: u16,
    b: ValueType
}

/*
Instructions are 1-3 words long and are fully defined by the first word.
In a basic instruction, the lower five bits of the first word of the instruction
are the opcode, and the remaining eleven bits are split into a five bit value b
and a six bit value a.
b is always handled by the processor after a, and is the lower five bits.
In bits (in LSB-0 format), a basic instruction has the format: aaaaaabbbbbooooo
*/
impl Instruction {
    fn new(i: u16) -> Instruction {
        let hop = i & 0b11111;
        let b_raw = (i >> 5) & 0b11111;
        let a_raw = i >> 10;

        let op: Option<Op> = FromPrimitive::from_u16(hop);

        let a = ValueType::new(a_raw);
        let b = ValueType::new(b_raw);

        match op {
            Some(op) => Instruction { op: op, a: a, b: b, b_raw: b_raw, a_raw: a_raw },
            None => fail!("Invalid instruction")
        }
    }
}

impl fmt::Show for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} - {:04x}, {} {:04x})",
               self.op, self.b, self.b_raw, self.a, self.a_raw)
    }
}

// FIXME: Make this less imperative!
#[allow(dead_code)]
pub fn load_program(file: Path) -> Vec<u16> {
    let mut fh = File::open(&file).unwrap();
    let mut eof = false;
    let mut data: Vec<u16> = Vec::new();

    while !eof {
        let b = fh.read_le_u16();
        match b {
            Ok(o) => data.push(o),
            _ => eof = true
        }
    };

    data
}

/*
fn main() {
    let c = CpuState::new();
    println!("{}", c);

    let prog = load_program(Path::new("brille.fil"));
    let k = c.set_program(&prog);

    println!("{}", k.step());
} */
