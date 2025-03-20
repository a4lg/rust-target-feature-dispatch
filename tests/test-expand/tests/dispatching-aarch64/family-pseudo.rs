fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if "aarch64" || "arm64ec" { // Not using family("aarch64") to force static dispatching.
            if "sve2" {
                "AArch64 + SVE2"
            }
        } else {
            "fallback"
        }
    };
}
