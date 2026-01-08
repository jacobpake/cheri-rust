use crate::spec::{
    Abi, Arch, Cc, LinkSelfContainedDefault, LinkerFlavor, Lld, Os, PanicStrategy, RelocModel, Target, TargetMetadata,
    TargetOptions, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        // The below `data_layout` is explicitly specified by the ilp32e ABI in LLVM. See also
        // `options.llvm_abiname`.
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128-pf200:64:64:64:32-A200-P200-G200".into(),
        llvm_target: "riscv32-unknown-cheriotrtos".into(),
        metadata: TargetMetadata {
            description: Some("CHERIoT RISC-V (RV32E ISA)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        arch: Arch::RiscV32,

        options: TargetOptions {
            abi: Abi::CHERIoT,
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: None,
            cpu: "cheriot".into(),
            llvm_abiname: "cheriot".into(),
            max_atomic_width: Some(64), // FIXME(jacobpake): Should be 32
            atomic_cas: true,
            singlethread: true, // FIXME(jacobpake): sensible for now
            features: "+32bit,+c,+e,+m,+xcheriot".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            families: cvs!["cheri", "cheriot"],
            os: Os::CHERIoTRTOS,
            is_like_cheri: true,
            executables: true,

            // FIXME(jacobpake): which of these are necessary?
            entry_name: "__rust_main".into(),
            dynamic_linking: false,
            link_self_contained: LinkSelfContainedDefault::True,
            generate_arange_section: false,
            crt_static_default: true,
            crt_static_respected: true,
            crt_static_allows_dylibs: false,

            default_address_space: rustc_abi::AddressSpace(200),
            ..Default::default()
        },
    }
}
