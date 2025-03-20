fn main() {
    let _: i32 = target_feature_dispatch::target_feature_dispatch! {
        // SPECIFIC: ARCH_COMPILER_VARIANT
        if "aarch64" || "arm" || "arm64ec" || "avr" || "bpf" || "csky" || "hexagon" || "loongarch64" || "m68k" || "mips" || "mips32r6" || "mips64" || "mips64r6" || "msp430" || "nvptx64" || "powerpc" || "powerpc64" || "riscv32" || "riscv64" || "s390x" || "sparc" || "sparc64" || "wasm32" || "wasm64" || "x86" || "x86_64" || "xtensa" {
            // ERROR: an expression must be enclosed to distinguish between
            //        this macro's special syntax and the regular expression.
            1
        } else {
            compile_error!(concat!(
                "This position should not be expanded ",
                "(maybe Rust compiler added an architecture?)"
            ));
        }
    };
}
