
use cpu::CpuState;

mod cpu;

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

    print!("{}", c);

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0x50);
}

#[test]
fn sub_next_word() {
    // SET A, 0x30
    // SUB A, 0x20
    let p: Vec<u16> = vec!(0x7c01, 0x30, 0x7c03, 0x20);
    let c = CpuState::new().set_program(&p).step().step();

    print!("{}", c);

    assert!(c.pc == 4);
    assert!(c.reg[0] == 0x10);
}
