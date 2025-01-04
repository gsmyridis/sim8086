use std::path::Path;

use lib8086::{Cpu, Decoder};

fn execute_test_file(filename: &str) -> Cpu {
    let buffer_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/execute")
        .join(filename);
    let buffer = std::fs::read(buffer_path).expect("Failed to read test file.");
    let decoder = Decoder::new(buffer);
    let i_queue = decoder.decode().expect("Decoding failed.");
    let mut cpu = Cpu::new();
    cpu.execute(&i_queue).expect("Execution failed.");
    cpu
}

#[test]
fn test_immediate_movs() {
    let cpu = execute_test_file("test_immediate_movs");

    assert_eq!(cpu.gen_regs.ax, 1u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bx, 2u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 3u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 4u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.sp, 5u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 6u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.si, 7u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.di, 8u16.to_le_bytes());

    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.sign);
    assert!(cpu.flags.parity);
}

#[test]
fn test_register_movs() {
    let cpu = execute_test_file("test_register_movs");

    assert_eq!(cpu.gen_regs.ax, 0x4411u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bx, 0x3344u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x6677u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x7788u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.sp, 0x4411u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x3344u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.si, 0x6677u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.di, 0x7788u16.to_le_bytes());

    assert_eq!(cpu.seg_regs.es, 0x6677u16.to_le_bytes());
    assert_eq!(cpu.seg_regs.ss, 0x4411u16.to_le_bytes());
    assert_eq!(cpu.seg_regs.ds, 0x3344u16.to_le_bytes());
}

#[test]
fn test_arithmetic() {
    let cpu = execute_test_file("test_num");

    assert_eq!(cpu.gen_regs.bx, 0xe102u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x0f01u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.sp, 0x03e6u16.to_le_bytes());

    assert!(cpu.flags.parity);
    assert!(cpu.flags.zero);
}

#[test]
fn test_flags() {
    let cpu = execute_test_file("test_flags");

    assert_eq!(cpu.gen_regs.bx, 0x9ca5u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x000au16.to_le_bytes());
    assert_eq!(cpu.gen_regs.sp, 0x0063u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x0062u16.to_le_bytes());

    assert!(cpu.flags.carry);
    assert!(cpu.flags.parity);
    assert!(cpu.flags.aux_carry);
    assert!(cpu.flags.sign);
}

#[test]
fn test_ip_reg() {
    let cpu = execute_test_file("test_ip_register");

    assert_eq!(cpu.gen_regs.bx, 0x07d0u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0xfce0u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x000eu16);
}

#[test]
fn test_cond_jumps_1() {
    let cpu = execute_test_file("test_cond_jumps_1");

    assert_eq!(cpu.gen_regs.bx, 0x0406u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x000eu16);

    assert!(cpu.flags.zero);
    assert!(cpu.flags.parity);
}

// #[test]
// fn test_cond_jumps_2() {
//     let cpu = execute_test_file("test_cond_jumps_2");
// 
//     assert_eq!(cpu.gen_regs.ax, 0x000du16.to_le_bytes());
//     assert_eq!(cpu.gen_regs.bx, 0xfffbu16.to_le_bytes());
//     assert_eq!(cpu.ip, 0x001cu16);
// 
//     assert!(cpu.flags.carry);
//     assert!(cpu.flags.aux_carry);
//     assert!(cpu.flags.sign);
// }

#[test]
fn test_memory_movs() {
    let cpu = execute_test_file("test_memory_movs");

    assert_eq!(cpu.gen_regs.bx, 0x0001u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x0002u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x000au16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x0004u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x0030u16);
}

#[test]
fn test_memory_num_loop() {
    let cpu = execute_test_file("test_memory_num_loop");

    assert_eq!(cpu.gen_regs.bx, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x0004u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x03e8u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.si, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x0023);

    assert!(cpu.flags.parity);
    assert!(cpu.flags.zero);
}
