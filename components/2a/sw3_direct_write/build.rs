use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=syswhispers/syscalls.c");
    println!("cargo:rerun-if-changed=syswhispers/syscalls.h");
    println!("cargo:rerun-if-changed=syswhispers/syscalls-asm.x64.asm");

    cc::Build::new()
        .file("syswhispers/syscalls.c")
        .compile("syscalls");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let asm_obj = out_dir.join("syscalls-asm.x64.obj");

    let llvm_prefix = std::env::var("LLVM_PREFIX")
        .unwrap_or_else(|_| "/opt/homebrew/opt/llvm".to_string());

    let llvm_ml = PathBuf::from(llvm_prefix).join("bin/llvm-ml");

    let status = Command::new(&llvm_ml)
        .args([
            "-m64",
            "-c",
            "syswhispers/syscalls-asm.x64.asm",
            "-Fo",
        ])
        .arg(&asm_obj)
        .status()
        .expect("failed to execute llvm-ml");

    assert!(status.success(), "llvm-ml failed");

    println!("cargo:rustc-link-arg={}", asm_obj.display());
}
