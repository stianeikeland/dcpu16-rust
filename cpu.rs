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

fn new_cpu() -> CpuState {
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

#[allow(dead_code)] // REMOVE ME!
fn print_state(cpu: CpuState) {
    println!("CPU State:\n-----------------------");
    print!("Reg: [");
    for r in cpu.reg.iter() {
        print!(" 0x{:04x}", *r);
    }
    println!(" ]");

    println!("PC: 0x{:04x}\nSP: 0x{:04x}\nEX: 0x{:04x}\nIA: 0x{:04x}",
             cpu.pc, cpu.sp, cpu.ex, cpu.ia);
    println!("-----------------------");
}

fn main() {
    let c = new_cpu();
    print_state(c);
}
