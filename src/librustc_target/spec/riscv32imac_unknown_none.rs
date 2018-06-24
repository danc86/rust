use spec::{LinkerFlavor, PanicStrategy, Target, TargetOptions, TargetResult};
use spec::abi::Abi;

pub fn target() -> TargetResult {
    Ok(Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".to_string(),
        llvm_target: "riscv32".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        target_os: "none".to_string(),
        target_env: "".to_string(),
        target_vendor: "unknown".to_string(),
        arch: "riscv".to_string(),
        linker_flavor: LinkerFlavor::Ld,


        options: TargetOptions {
            linker: Some("ld.lld".to_string()),
            cpu: "generic-rv32".to_string(),
            max_atomic_width: Some(32),
            features: "+m,+a,+c".to_string(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: "static".to_string(),
            abi_blacklist: vec![
                Abi::Cdecl,
                Abi::Stdcall,
                Abi::Fastcall,
                Abi::Vectorcall,
                Abi::Thiscall,
                Abi::Aapcs,
                Abi::Win64,
                Abi::SysV64,
                Abi::PtxKernel,
                Abi::Msp430Interrupt,
                Abi::X86Interrupt,
            ],
            .. Default::default()
        },
    })
}
