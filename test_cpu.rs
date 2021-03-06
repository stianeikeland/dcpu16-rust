
use cpu::CpuState;

mod cpu;

/*
--- Basic opcodes (5 bits) ----------------------------------------------------
 C | VAL  | NAME     | DESCRIPTION
---+------+----------+---------------------------------------------------------
 - | 0x00 | n/a      | special instruction - see below
 1 | 0x01 | SET b, a | sets b to a
 2 | 0x02 | ADD b, a | sets b to b+a, sets EX to 0x0001 if there's an overflow,
   |      |          | 0x0 otherwise
 2 | 0x03 | SUB b, a | sets b to b-a, sets EX to 0xffff if there's an underflow,
   |      |          | 0x0 otherwise
 2 | 0x04 | MUL b, a | sets b to b*a, sets EX to ((b*a)>>16)&0xffff (treats b,
   |      |          | a as unsigned)
 2 | 0x05 | MLI b, a | like MUL, but treat b, a as signed
 3 | 0x06 | DIV b, a | sets b to b/a, sets EX to ((b<<16)/a)&0xffff. if a==0,
   |      |          | sets b and EX to 0 instead. (treats b, a as unsigned)
 3 | 0x07 | DVI b, a | like DIV, but treat b, a as signed. Rounds towards 0
 3 | 0x08 | MOD b, a | sets b to b%a. if a==0, sets b to 0 instead.
 3 | 0x09 | MDI b, a | like MOD, but treat b, a as signed. (MDI -7, 16 == -7)
 1 | 0x0a | AND b, a | sets b to b&a
 1 | 0x0b | BOR b, a | sets b to b|a
 1 | 0x0c | XOR b, a | sets b to b^a
 1 | 0x0d | SHR b, a | sets b to b>>>a, sets EX to ((b<<16)>>a)&0xffff
   |      |          | (logical shift)
 1 | 0x0e | ASR b, a | sets b to b>>a, sets EX to ((b<<16)>>>a)&0xffff
   |      |          | (arithmetic shift) (treats b as signed)
 1 | 0x0f | SHL b, a | sets b to b<<a, sets EX to ((b<<a)>>16)&0xffff
 2+| 0x10 | IFB b, a | performs next instruction only if (b&a)!=0
 2+| 0x11 | IFC b, a | performs next instruction only if (b&a)==0
 2+| 0x12 | IFE b, a | performs next instruction only if b==a
 2+| 0x13 | IFN b, a | performs next instruction only if b!=a
 2+| 0x14 | IFG b, a | performs next instruction only if b>a
 2+| 0x15 | IFA b, a | performs next instruction only if b>a (signed)
 2+| 0x16 | IFL b, a | performs next instruction only if b<a
 2+| 0x17 | IFU b, a | performs next instruction only if b<a (signed)
 - | 0x18 | -        |
 - | 0x19 | -        |
 3 | 0x1a | ADX b, a | sets b to b+a+EX, sets EX to 0x0001 if there is an over-
   |      |          | flow, 0x0 otherwise
 3 | 0x1b | SBX b, a | sets b to b-a+EX, sets EX to 0xFFFF if there is an under-
   |      |          | flow, 0x0 otherwise
 - | 0x1c | -        |
 - | 0x1d | -        |
 2 | 0x1e | STI b, a | sets b to a, then increases I and J by 1
 2 | 0x1f | STD b, a | sets b to a, then decreases I and J by 1
---+------+----------+----------------------------------------------------------
*/

#[test]
fn set_reg_from_next_word() {
    let p: Vec<u16> = vec!(0x7c01, 0x30); // SET A, 0x30
    let c = CpuState::new().set_program(&p).step();

    assert!(c.pc == 2);
    assert!(c.reg[0] == 0x30);
}

#[test]
fn add_from_next_word() {
    // SET A, 0x30
    // ADD A, 0x20
    let p: Vec<u16> = vec!(0x7c01, 0x30, 0x7c02, 0x20);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0x50);
}

#[test]
fn sub_next_word() {
    // SET A, 0x30
    // SUB A, 0x20
    let p: Vec<u16> = vec!(0x7c01, 0x30, 0x7c03, 0x20);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0x10);
}

#[test]
fn mul_next_word() {
    // SET A, 0x30
    // MUL A, 0x20
    let p: Vec<u16> = vec!(0x7c01, 0x30, 0x7c04, 0x20);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0x600);
}

//TODO: add signed mul

#[test]
fn div_next_word() {
    // SET A, 0x30
    // DIV A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x30, 0x7c06, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0x18);
}

//TODO: add signed div

#[test]
fn mod_next_word() {
    // SET A, 0x31
    // MOD A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c08, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 1);
}

#[test]
fn mod_0_should_be_0() {
    // SET A, 0x31
    // MOD A, 0
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c08, 0);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0);
}

//TODO: add signed mod

#[test]
fn and_next_word() {
    // SET A, 0x31
    // AND A, 0x15
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c0a, 0x15);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 17);
}

#[test]
fn or_next_word() {
    // SET A, 0x31
    // BOR A, 0x15
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c0b, 0x15);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 53);
}

#[test]
fn xor_next_word() {
    // SET A, 0x31
    // XOR A, 0x15
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c0c, 0x15);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 36);
}

#[test]
fn shr_next_word() {
    // SET A, 0x31
    // SHR A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c0d, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 12);
}

#[test]
fn asr_next_word() {
    // SET A, 0x31
    // ASR A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x31, 0x7c0e, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    assert!(c.pc == 4);
    assert!(c.reg[0] == 196);
}

/*
 2+| 0x10 | IFB b, a | performs next instruction only if (b&a)!=0
 2+| 0x11 | IFC b, a | performs next instruction only if (b&a)==0
 2+| 0x12 | IFE b, a | performs next instruction only if b==a
 2+| 0x13 | IFN b, a | performs next instruction only if b!=a
 2+| 0x14 | IFG b, a | performs next instruction only if b>a
 2+| 0x15 | IFA b, a | performs next instruction only if b>a (signed)
 2+| 0x16 | IFL b, a | performs next instruction only if b<a
 2+| 0x17 | IFU b, a | performs next instruction only if b<a (signed)
*/

#[test]
fn ifb_and_false_should_skip_next_instr() {
    // SET A, 0x1
    // IFB A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x1, 0x7c10, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    println!("{}", c);

    assert!(c.pc == 5);
}

#[test]
fn ifc_nand_false_should_skip_next_instr() {
    // SET A, 0x1
    // IFC A, 0x1
    let p: Vec<u16> = vec!(0x7c01, 0x1, 0x7c11, 0x1);
    let c = CpuState::new().set_program(&p).step().step();

    println!("{}", c);

    assert!(c.pc == 5);
}

#[test]
fn ife_not_eql_should_skip_next_instr() {
    // SET A, 0x1
    // IFE A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x1, 0x7c12, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    println!("{}", c);

    assert!(c.pc == 5);
}

#[test]
fn ifn_eql_should_skip_next_instr() {
    // SET A, 0x1
    // IFN A, 0x1
    let p: Vec<u16> = vec!(0x7c01, 0x1, 0x7c13, 0x1);
    let c = CpuState::new().set_program(&p).step().step();

    println!("{}", c);

    assert!(c.pc == 5);
}

#[test]
fn ifg_not_greater_should_skip_next_instr() {
    // SET A, 0x1
    // IFG A, 0x2
    let p: Vec<u16> = vec!(0x7c01, 0x1, 0x7c14, 0x2);
    let c = CpuState::new().set_program(&p).step().step();

    println!("{}", c);

    assert!(c.pc == 5);
}

// TODO: IFA

#[test]
fn ifl_not_greater_should_skip_next_instr() {
    // SET A, 0x2
    // IFL A, 0x1
    let p: Vec<u16> = vec!(0x7c01, 0x2, 0x7c16, 0x1);
    let c = CpuState::new().set_program(&p).step().step();

    println!("{}", c);

    assert!(c.pc == 5);
}

// TODO: IFU
