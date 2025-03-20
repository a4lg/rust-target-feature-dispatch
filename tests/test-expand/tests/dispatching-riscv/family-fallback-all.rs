fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if family("riscv") {
            if "zba" {
                "RISC-V + Zba"
            }
        } else {
            "fallback"
        }
    };
}
