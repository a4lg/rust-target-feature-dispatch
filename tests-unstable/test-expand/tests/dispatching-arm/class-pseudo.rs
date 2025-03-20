#![cfg_attr(feature = "unstable", feature(stdarch_arm_feature_detection))]
fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if "aarch64" || "arm64ec" || "arm" { // Not using class("arm") to complare implementations.
            if "dotprod" {
                "Arm + DOTPROD"
            }
        } else {
            "fallback"
        }
    };
}
