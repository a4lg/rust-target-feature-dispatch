fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if "riscv32" || "riscv64" { // Not using family("riscv") to force static dispatching.
            if "zba" {
                "RISC-V + Zba"
            }
        } else {
            "fallback"
        }
    };
}
