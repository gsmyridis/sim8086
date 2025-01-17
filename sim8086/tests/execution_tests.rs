use std::path::Path;

use lib8086::Cpu;

fn execute_file(filename: &str) -> Cpu {
    let buffer_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/execute")
        .join(filename);
    let buffer = std::fs::read(buffer_path).expect("Failed to read test file.");
    let mut cpu = Cpu::new();
    cpu.load_instructions(&buffer);
    cpu.execute().unwrap();
    cpu
}

#[test]
fn test_immediate_movs() {
    let cpu = execute_file("immediate_movs");

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
    let cpu = execute_file("register_movs");

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
    let cpu = execute_file("arithmetic");

    assert_eq!(cpu.gen_regs.bx, 0xe102u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x0f01u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.sp, 0x03e6u16.to_le_bytes());

    assert!(cpu.flags.parity);
    assert!(cpu.flags.zero);
}

#[test]
fn test_flags() {
    let cpu = execute_file("flags");

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
    let cpu = execute_file("ip_register");

    assert_eq!(cpu.gen_regs.bx, 0x07d0u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0xfce0u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x000eu16);
}

#[test]
fn test_cond_jumps_1() {
    let cpu = execute_file("cond_jumps_1");

    assert_eq!(cpu.gen_regs.bx, 0x0406u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x000eu16);

    assert!(cpu.flags.zero);
    assert!(cpu.flags.parity);
}

#[test]
fn test_cond_jumps_2() {
    let cpu = execute_file("cond_jumps_2");

    assert_eq!(cpu.gen_regs.ax, 0x000du16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bx, 0xfffbu16.to_le_bytes());
    assert_eq!(cpu.ip, 0x001cu16);

    assert!(cpu.flags.carry);
    assert!(cpu.flags.aux_carry);
    assert!(cpu.flags.sign);
}

#[test]
fn test_memory_movs() {
    let cpu = execute_file("memory_movs");

    assert_eq!(cpu.gen_regs.bx, 0x0001u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x0002u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x000au16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x0004u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x0030u16);
}

#[test]
fn test_memory_num_loop() {
    let cpu = execute_file("mem_num_loop");

    assert_eq!(cpu.gen_regs.bx, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.cx, 0x0004u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x03e8u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.si, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x0023);

    assert!(cpu.flags.parity);
    assert!(cpu.flags.zero);
}

#[test]
fn test_add_loop() {
    let cpu = execute_file("add_loop");

    assert_eq!(cpu.gen_regs.bx, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x0006u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x03e6u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x0021u16);
}

#[test]
fn test_draw_rectangle() {
    let cpu = execute_file("draw_rectangle");

    assert_eq!(cpu.gen_regs.cx, 0x0040u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.dx, 0x0040u16.to_le_bytes());
    assert_eq!(cpu.gen_regs.bp, 0x4100u16.to_le_bytes());
    assert_eq!(cpu.ip, 0x0026u16);
}

// #[test]
// fn test_draw_rectangle_2() {
//     let cpu = execute_file("draw_rectangle_2");
//
//     assert_eq!(cpu.gen_regs.cx, 0x4004u16.to_le_bytes());
//     assert_eq!(cpu.gen_regs.bp, 0x02fcu16.to_le_bytes());
//     assert_eq!(cpu.ip, 0x0044u16);
// }
