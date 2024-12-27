use std::io::Read;
use std::process::Command;
use std::sync::OnceLock;

use tempdir::TempDir;

static NASM_CHECK: OnceLock<()> = OnceLock::new();

fn ensure_nasm_installed() {
    NASM_CHECK.get_or_init(|| {
        let status = Command::new("nasm")
            .arg("--version")
            .status()
            .expect("Failed to run `nasm --version`.");

        if !status.success() {
            panic!("In order to run the tests you must have NASM installed in your system.");
        }
    });
}

#[test]
fn test_decode() {
    ensure_nasm_installed();

    let dir = std::fs::read_dir("tests/data/")
        .expect("You must run the tests from the repo's base directory");
    let temp_dir = TempDir::new("output").expect("Failed to create temporary directory");

    for entry in dir {
        let entry = entry.expect("Failed to unwrap DirEntry");
        let bin_path = entry.path();
        let bin_filename = bin_path.file_name().expect("Entry is not directory");
        let mut asm_path = temp_dir.path().join(bin_filename);
        asm_path.set_extension("asm");

        // Decode byte-code with sim8086 decode.
        let status = Command::new("target/debug/sim8086")
            .arg("decode")
            .arg("--output")
            .arg(asm_path.as_os_str())
            .arg(entry.path().as_os_str())
            .status()
            .expect("Failed to get status");
        assert!(
            status.success(),
            "Failed to execute the `sim8086 decode` command."
        );

        // Encode decoded assembly into byte-code.
        let status = Command::new("nasm")
            .arg("-o")
            .arg(asm_path.with_extension(""))
            .arg(asm_path.as_os_str())
            .status()
            .expect("Failed to get status");
        assert!(status.success(), "Failed to execute `nasm` command.");

        // Reads the original byte-code.
        let mut original_bin =
            std::fs::File::open(entry.path()).expect("Failed to read original byte-code file.");
        let mut original_buf = Vec::new();
        original_bin
            .read_to_end(&mut original_buf)
            .expect("Failed to read original byte-code file.");

        // Read the byte-code from the encoded decode.
        let mut output_bin =
            std::fs::File::open(entry.path()).expect("Failed to read original byte-code file.");
        let mut output_buf = Vec::new();
        output_bin
            .read_to_end(&mut output_buf)
            .expect("Failed to read output byte-code file.");

        assert_eq!(output_buf, original_buf);
    }

    temp_dir
        .close()
        .expect("Failed to close temporary directory");
}
